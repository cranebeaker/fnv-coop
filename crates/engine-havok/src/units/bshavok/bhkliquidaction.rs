//! `BSHavok/bhkliquidaction.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkLiquidAction` (0x40 bytes) is a `bhkUnaryAction` (`phkObject` at `+8`, `pInfo` at
//! `+0xC`) that drives a target object's neighbours: the target `NiAVObject` it follows is
//! kept in the map at `011b2190` (an `NiTMap` from the target to a `BSSimpleList` of the
//! actions that follow it; RTTI-confirmed on the vtables `010c91a8` and `010c91c8`).
//!
//! The functions that allocate or tear down Havok objects raise the per-thread allocation
//! word at `TLS + 0x2b4` to 0x10 for their duration and put the old value back. The
//! exception-unwinding frames (`FS:[0]` registrations) and the `/GS` cookie check of the
//! functions marked `Translation incomplete` are not modeled.

#[allow(unused_imports)]
use crate::prelude::*;

use super::bhklimitedhingeconstraint::{
    allocate_sized, enter_scope, push_viewer_string, REMOVE_REFERENCE,
};

/// `00aa3e40`: thiscall on the pool object at `011f6238`, one word (the
/// size); returns the new block, or 0.
const POOL_ALLOCATE: u32 = 0x00aa_3e40;
/// The pool object the allocation is requested from.
const POOL_011F6238: u32 = 0x011f_6238;
/// Pool free (`00aa4060`, thiscall on the pool object, one word: the block).
const POOL_FREE: u32 = 0x00aa_4060;
/// CRT-style free (`00aa10f0`, cdecl, one word).
const FREE: u32 = 0x00aa_10f0;
/// Allocation of a size (`00aa13e0`, cdecl, one word).
const ALLOCATE: u32 = 0x00aa_13e0;
/// The map of target objects to the lists of actions that follow them.
const TARGET_MAP: u32 = 0x011b_2190;
/// `NiTMap::GetAt` (`00a53c30`, `(key, out)`), `SetAt` (`00b57ea0`, `(key, value)`) and
/// `RemoveAt` (`00e91fd0`, `(key)`), thiscall on the map; `NiTMapBase::RemoveAll` (`00b601a0`).
const MAP_GET_AT: u32 = 0x00a5_3c30;
const MAP_SET_AT: u32 = 0x00b5_7ea0;
const MAP_REMOVE_AT: u32 = 0x00e9_1fd0;
const MAP_REMOVE_ALL: u32 = 0x00b6_01a0;
/// The list node clear used when a node is unlinked (`00b4df30`, thiscall, no words).
const LIST_CLEAR: u32 = 0x00b4_df30;
/// `bhkUnaryAction` base methods: `~` (`00c86720`), `LoadBinary` (`00c86790`), `SaveBinary`
/// (`00c868b0`), `CopyMembers` (`00c86800`, `(clone, process_info)`), `CopyCinfo`
/// (`00c867d0`) and `GetViewerStrings` (`00c86b80`); `bhkSerializable::Kill` (`00c876a0`),
/// `CloneCritLock` (`00c876f0`) and `CloneCritUnlock` (`00c87700`).
const UNARY_LOAD_BINARY: u32 = 0x00c8_6790;
const UNARY_SAVE_BINARY: u32 = 0x00c8_68b0;
const UNARY_DESTRUCT: u32 = 0x00c8_6720;
const UNARY_COPY_MEMBERS: u32 = 0x00c8_6800;
const UNARY_COPY_CINFO: u32 = 0x00c8_67d0;
const UNARY_GET_VIEWER_STRINGS: u32 = 0x00c8_6b80;
const SERIALIZABLE_KILL: u32 = 0x00c8_76a0;
const CLONE_CRIT_LOCK: u32 = 0x00c8_76f0;
const CLONE_CRIT_UNLOCK: u32 = 0x00c8_7700;
/// The `bhkLiquidAction` constructor (`00cbc550`, `ECX` = the block, returns it).
const CONSTRUCT: u32 = 0x00cb_c550;
/// `hkLiquidAction` constructor (`00d2c910`, `(info + 4, 0)`).
const HK_LIQUID_ACTION_CONSTRUCT: u32 = 0x00d2_c910;
/// `bhkWorldObject::GetProperty` (`00c85ef0`, cdecl `(out, object, property)`, answers a
/// pointer to the value) with the property id used here, and the lookup of the player
/// reference `00653290` (`thiscall` on the property's value, one word) with its argument.
const GET_PROPERTY: u32 = 0x00c8_5ef0;
const PROPERTY_ID: u32 = 0x0126_7b70;
const FIND_BY_KEY: u32 = 0x0065_3290;
const FIND_KEY: u32 = 0x0120_43f8;
/// `NiGetViewerString` (`00a7d330`, cdecl, one word): the viewer string of an `NiRTTI`.
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
/// The `NiRTTI` global whose viewer string heads the action's list.
const ACTION_RTTI: u32 = 0x0126_8420;
/// Instance counter of `bhkLiquidAction`; the class vtable.
const COUNT_LIQUID_ACTION: u32 = 0x0126_841c;
const VTABLE_LIQUID_ACTION: u32 = 0x0108_f1bc;
/// Vtables of `NiTMapBase<...>` and of `NiTMap<...>` over the same map items.
const VTABLE_TI_MAP_BASE: u32 = 0x010c_91a8;
const VTABLE_TI_MAP: u32 = 0x010c_91c8;
/// `Update` constants and helpers. The zero vector the force starts from and the stick
/// point is compared with, three vectors/`float`s read from the exe, and the callees.
const ZERO_VECTOR: u32 = 0x011f_426c;
const DEFAULT_VECTOR: u32 = 0x0126_7e30;
const ZERO_FLOAT: u32 = 0x0101_1d78;
const HALF_FLOAT: u32 = 0x0101_6248;
const NEGATIVE_HALF_FLOAT: u32 = 0x0102_295c;
const CLAMP_HIGH_DOUBLE: u32 = 0x0101_2070;
const CLAMP_LOW_DOUBLE: u32 = 0x0101_1598;
const SPEED_FACTOR_DOUBLE: u32 = 0x0101_a6b0;
const STICK_DECAY_DOUBLE: u32 = 0x0106_b9e8;
const FACTOR_FLOAT_MOVING: u32 = 0x011b_2184;
const FACTOR_FLOAT_RESTING: u32 = 0x011b_2180;
const GRAVITY_SCALE_A: u32 = 0x011b_218c;
const GRAVITY_SCALE_B: u32 = 0x011b_217c;
const NEIGHBOR_VELOCITY_SCALE: u32 = 0x011b_2188;
const NEIGHBOR_RANGE_SCALE: u32 = 0x011b_2178;
const STICK_DIRECTION: u32 = 0x011a_9484;
const FORCE_SCALE: u32 = 0x0126_8428;
const GRAVITY_FLAG: u32 = 0x0126_842c;
/// `ahkpEntity`-side helpers: the count of the entity's contact list (`00c9c380`, `ECX` =
/// the body) and its element getter (`00c9c390`, one word), `hkpEntity::activate`
/// (`00c9c1d0`), `hkpRigidBody::setLinearVelocity` (`005616d0`, one word) and the empty
/// bracket function (`00a29680`) called around both.
const CONTACT_COUNT: u32 = 0x00c9_c380;
const CONTACT_GET: u32 = 0x00c9_c390;
const ACTIVATE: u32 = 0x00c9_c1d0;
const SET_LINEAR_VELOCITY: u32 = 0x0056_16d0;
const BRACKET: u32 = 0x00a2_9680;
/// `NiPoint3` equality of the point at `ECX` with a pointer (`004390c0`), the copies into a
/// `NiPoint3` (`00458620`, cdecl `(destination, source)`) and from a Havok vector
/// (`004a3970`, cdecl `(destination, source)`), `NiPoint3::UnitizeGetLength` (`00457910`,
/// answers the length in `ST0`), and the x87 helpers `sqrt` (`00ec6040`) and `exp`
/// (`00ecae6c`), whose argument is the leading `f64`.
const POINT_EQUAL: u32 = 0x0043_90c0;
const COPY_POINT: u32 = 0x0045_8620;
const POINT_FROM_VECTOR: u32 = 0x004a_3970;
const UNITIZE_GET_LENGTH: u32 = 0x0045_7910;
const X87_SQRT: u32 = 0x00ec_6040;
const X87_EXP: u32 = 0x00ec_ae6c;

layout! {
    /// `bhkLiquidAction` (Xbox PDB names, PC offsets confirmed by `CreateCinfo` and `Init`),
    /// 0x40 bytes.
    pub struct BhkLiquidAction: 0x40 {
        0x08 phkObject: u32,
        0x0C pInfo: u32,
        0x10 pTargetObject: u32,
        0x14 fNeighborDistance: f32,
        0x18 fNeighborStrength: f32,
        0x1C fInitialStickForce: f32,
        0x20 fStickStrength: f32,
        0x24 StickPoint: u32,
        0x30 StickNormal: u32,
        0x3C bFirstUpdate: u8,
    }
}

// Translated from 00cbc650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkLiquidAction::LoadBinary` (Xbox PDB): the `bhkUnaryAction` load (`00c86790`), then,
/// for a stream not in PC byte order (`stream + 0x1E5` not 1), swaps the four words at
/// `+8..+0x14` of the cinfo that `CreateCinfo` (slot `0xB4`) answers.
pub fn bhk_liquid_action_load_binary(e: &mut Engine, this: Ptr<BhkLiquidAction>, stream: u32) {
    e.call(UNARY_LOAD_BINARY, &args![this, stream]);
    if e.mem.u8(stream + 0x1e5) != 1 {
        let info = e
            .with_stack(4, |e, created| e.vcall(this.addr(), 0xb4, &args![created]))
            .u32();
        swap_cinfo_words(e, info);
    }
}

/// Byte-swaps the four words at `+8..+0x14` of `info`.
fn swap_cinfo_words(e: &mut Engine, info: u32) {
    for offset in [8, 0xc, 0x10, 0x14] {
        let word = e.mem.u32(info + offset);
        e.mem.set_u32(info + offset, word.swap_bytes());
    }
}

// Translated from 00cbc6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkLiquidAction::SaveBinary` (Xbox PDB): takes the cinfo from `CreateCinfo` (slot
/// `0xB4`), swaps its four words at `+8..+0x14` when the stream is not in PC byte order
/// (`stream + 0x1E4` not 1), runs the `bhkUnaryAction` save (`00c868b0`) and swaps them
/// back.
pub fn bhk_liquid_action_save_binary(e: &mut Engine, this: Ptr<BhkLiquidAction>, stream: u32) {
    let info = e
        .with_stack(4, |e, created| e.vcall(this.addr(), 0xb4, &args![created]))
        .u32();
    if e.mem.u8(stream + 0x1e4) != 1 {
        swap_cinfo_words(e, info);
    }
    e.call(UNARY_SAVE_BINARY, &args![this, stream]);
    if e.mem.u8(stream + 0x1e4) != 1 {
        swap_cinfo_words(e, info);
    }
}

// Translated from 00cbc720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of the map base class (vtable `010c91a8`, RTTI `NiTMapBase<...>`):
/// sets that vtable, removes every item (`00b601a0`) and frees the bucket array at `+8`.
pub fn fn_00cbc720(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_TI_MAP_BASE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let buckets = e.mem.u32(this.addr() + 8);
    e.call(FREE, &args![buckets]);
}

// Translated from 00cbc740 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cbc740 - the FS:[0] exception-unwinding frame is not modeled.
/// Destructor body of the map (vtable `010c91c8`, RTTI `NiTMap<...>`): sets that vtable,
/// removes every item (`00b601a0`), sets the base vtable (`010c91a8`), removes again and
/// frees the bucket array at `+8`.
pub fn fn_00cbc740(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_TI_MAP);
    e.call(MAP_REMOVE_ALL, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE_TI_MAP_BASE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let buckets = e.mem.u32(this.addr() + 8);
    e.call(FREE, &args![buckets]);
}

// Translated from 00cbc7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (the map places it in `bhkliquidaction.obj`). Thiscall, one
/// word (`RET 4`): `this` is a head word plus a link at `+4`, and `source`
/// points at the word to put in. Nothing happens when `[source]` is 0.
/// When `[this]` is empty it just takes the word. Otherwise an 8-byte node
/// is allocated from the pool and the old head is pushed on the list
/// (`node = {old head, old [this + 4]}`, `[this + 4] = node`) before
/// `[this]` takes the new word. When the allocation fails the code writes
/// the link to address 4 (a null pointer plus 4), which faults in the game,
/// and so it does here; the rest of that path is kept after it.
pub fn fn_00cbc7b0(e: &mut Engine, this: Ptr, source: Ptr) {
    let mut word = e.mem.u32(source.addr());
    if word == 0 {
        return;
    }
    if e.mem.u32(this.addr()) != 0 {
        let node = e.call(POOL_ALLOCATE, &args![POOL_011F6238, 8u32]).u32();
        if node != 0 {
            let head = e.mem.u32(this.addr());
            e.mem.set_u32(node, head);
            e.mem.set_u32(node + 4, 0);
            let link = e.mem.u32(this.addr() + 4);
            e.mem.set_u32(node + 4, link);
            e.mem.set_u32(this.addr() + 4, node);
            let new_head = e.mem.u32(source.addr());
            e.mem.set_u32(this.addr(), new_head);
            return;
        }
        let link = e.mem.u32(this.addr() + 4);
        e.mem.set_u32(4, link);
        e.mem.set_u32(this.addr() + 4, 0);
        word = e.mem.u32(source.addr());
    }
    e.mem.set_u32(this.addr(), word);
}

// Translated from 00cbc810 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: removes one value from a `BSSimpleList` (head word at `+0`, next node
/// at `+4`; `this` is the head node, `source` points at the value to remove). Nothing
/// happens when that value is 0 or the list is empty. The first node holding the value
/// is unlinked: when it is the head, the second node's value and link move into the head
/// and the second node is released; the head alone becomes empty. Released nodes are
/// cleared (`00b4df30`) and returned to the pool (`00aa4060`).
pub fn fn_00cbc810(e: &mut Engine, this: Ptr, source: Ptr) {
    let value = e.mem.u32(source.addr());
    if value == 0 {
        return;
    }
    let head = this.addr();
    let second = e.mem.u32(head + 4);
    if second == 0 && e.mem.u32(head) == 0 {
        return;
    }
    let mut previous = head;
    let mut node = head;
    while e.mem.u32(node) != value {
        previous = node;
        node = e.mem.u32(node + 4);
        if node == 0 {
            return;
        }
    }
    if node == head {
        if second == 0 {
            e.mem.set_u32(head, 0);
            return;
        }
        let next = e.mem.u32(second + 4);
        e.mem.set_u32(head + 4, next);
        let second_value = e.mem.u32(second);
        e.mem.set_u32(head, second_value);
        e.mem.set_u32(second + 4, 0);
        e.call(LIST_CLEAR, &args![second]);
        e.call(POOL_FREE, &args![POOL_011F6238, second]);
    } else {
        let next = e.mem.u32(node + 4);
        e.mem.set_u32(previous + 4, next);
        e.mem.set_u32(node + 4, 0);
        e.call(LIST_CLEAR, &args![node]);
        e.call(POOL_FREE, &args![POOL_011F6238, node]);
    }
}

// Translated from 00cbc8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<...>::_scalar_deleting_destructor_` (Xbox PDB): the base destructor body
/// (vtable `010c91a8`, remove all, free the buckets) and, when bit 0 of `flags` is set,
/// the block is returned to the pool. Answers `this`.
pub fn ni_t_map_base_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    fn_00cbc720(e, this);
    if flags & 1 != 0 {
        e.call(POOL_FREE, &args![POOL_011F6238, this]);
    }
    this
}

// Translated from 00cbc8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: the index of the last byte equal to `value` in the byte array whose data
/// pointer is at `+0xC` and count at `+0x10`, or -1 (the count minus one when it is
/// negative).
pub fn fn_00cbc8f0(e: &mut Engine, this: Ptr, value: u32) -> i32 {
    let mut index = (e.mem.u32(this.addr() + 0x10) as i32).wrapping_sub(1);
    if index < 0 {
        return index;
    }
    let data = e.mem.u32(this.addr() + 0xc);
    loop {
        if u32::from(e.mem.u8(data.wrapping_add(index as u32))) == value {
            return index;
        }
        index = index.wrapping_sub(1);
        if index < 0 {
            return index;
        }
    }
}

// Translated from 00cbc920 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cbc920 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkLiquidAction::~bhkLiquidAction` (Xbox PDB): sets the class vtable; with the
/// allocation-scope word at 0x10, takes this action out of the list the target-object map
/// holds for its target (`00cbc810`) and, when that list is then empty, removes the map
/// entry and releases the list; runs `bhkSerializable::Kill` (`00c876a0`), restores the
/// scope word, takes one off the counter `0126841c` and runs the `bhkUnaryAction`
/// destructor (`00c86720`).
pub fn bhk_liquid_action_destruct(e: &mut Engine, this: Ptr<BhkLiquidAction>) {
    let me = this.addr();
    e.mem.set_u32(me, VTABLE_LIQUID_ACTION);
    let (word, saved) = enter_scope(e);
    let target = e.get(this, BhkLiquidAction::pTargetObject);
    if target != 0 {
        let list = map_get(e, target);
        if list != 0 {
            e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), me);
                fn_00cbc810(e, Ptr::new(list), slot);
            });
            if e.mem.u32(list + 4) == 0 && e.mem.u32(list) == 0 {
                let key = e.get(this, BhkLiquidAction::pTargetObject);
                e.call(MAP_REMOVE_AT, &args![TARGET_MAP, key]);
                e.call(LIST_CLEAR, &args![list]);
                e.call(POOL_FREE, &args![POOL_011F6238, list]);
            }
        }
    }
    e.call(SERIALIZABLE_KILL, &args![this]);
    e.mem.set_u32(word, saved);
    let count = e.mem.u32(COUNT_LIQUID_ACTION);
    e.mem.set_u32(COUNT_LIQUID_ACTION, count.wrapping_sub(1));
    e.call(UNARY_DESTRUCT, &args![this]);
}

/// The list the target-object map holds for `key`, or 0 (`GetAt` into a zeroed word).
fn map_get(e: &mut Engine, key: u32) -> u32 {
    e.with_stack(4, |e, out| {
        e.mem.set_u32(out.addr(), 0);
        e.call(MAP_GET_AT, &args![TARGET_MAP, key, out]);
        e.mem.u32(out.addr())
    })
}

// Translated from 00cbca20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cbca20 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkLiquidAction::CreateClone` (Xbox PDB), vtable slot `0x48`: inside the clone lock
/// (`00c876f0` / `00c87700`) and with the allocation-scope word at 0x10, allocates 0x40
/// bytes (`00aa13e0`) and constructs a `bhkLiquidAction` there (`00cbc550`) when that
/// succeeded, then copies the members (`00c86800`, `(clone, process_info)`). Answers the
/// clone (0 when the allocation failed).
pub fn bhk_liquid_action_create_clone(
    e: &mut Engine,
    this: Ptr<BhkLiquidAction>,
    process_info: u32,
) -> u32 {
    let (word, saved) = enter_scope(e);
    e.call(CLONE_CRIT_LOCK, &args![]);
    let block = e.call(ALLOCATE, &args![0x40u32]).u32();
    let clone = if block != 0 {
        e.call(CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.call(UNARY_COPY_MEMBERS, &args![this, clone, process_info]);
    e.call(CLONE_CRIT_UNLOCK, &args![]);
    e.mem.set_u32(word, saved);
    clone
}

// Translated from 00cbcad0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cbcad0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkLiquidAction::CreateCinfo` (Xbox PDB), vtable slot `0xB4`: `created` (a byte) is
/// set to whether `pInfo` was empty. If so, a 0x28-byte block is taken from the pool
/// (`00aa3e40`; the result is not checked) and the cinfo is the 16-byte aligned address
/// inside it (the pad count is kept in the byte before it). Its `+0`/`+4` are cleared and
/// `+8`, `+0xC`, `+0x10`, `+0x14` take the `float`s at `01030020`, `0101e580`, `0101e6ec`
/// and `0101e580` again; `pInfo` takes it and, when the Havok object at `+8` exists,
/// `bhkUnaryAction`'s `CopyCinfo` (`00c867d0`) is given it and the action's `+0x1C`,
/// `+0x20`, `+0x14` and `+0x18` are copied into the cinfo's `+8..+0x14`. Answers
/// `pInfo`. The allocation-scope word is 0x10 meanwhile.
pub fn bhk_liquid_action_create_cinfo(
    e: &mut Engine,
    this: Ptr<BhkLiquidAction>,
    created: Ptr,
) -> u32 {
    let me = this.addr();
    let (word, saved) = enter_scope(e);
    let empty = e.get(this, BhkLiquidAction::pInfo) == 0;
    e.mem.set_u8(created.addr(), u8::from(empty));
    if empty {
        let block = e.call(POOL_ALLOCATE, &args![POOL_011F6238, 0x28u32]).u32();
        let pad = 0x10 - (block & 0xf) as u8;
        let info = block.wrapping_add(u32::from(pad));
        e.mem.set_u8(info.wrapping_sub(1), pad);
        e.mem.set_u32(info, 0);
        copy_float(e, 0x0103_0020, info + 8);
        e.mem.set_u32(info + 4, 0);
        copy_float(e, 0x0101_e580, info + 0xc);
        copy_float(e, 0x0101_e6ec, info + 0x10);
        copy_float(e, 0x0101_e580, info + 0x14);
        e.set(this, BhkLiquidAction::pInfo, info);
        if e.get(this, BhkLiquidAction::phkObject) != 0 {
            e.call(UNARY_COPY_CINFO, &args![this, info]);
            copy_float(e, me + 0x1c, info + 8);
            copy_float(e, me + 0x20, info + 0xc);
            copy_float(e, me + 0x14, info + 0x10);
            copy_float(e, me + 0x18, info + 0x14);
        }
    }
    let info = e.get(this, BhkLiquidAction::pInfo);
    e.mem.set_u32(word, saved);
    info
}

/// Copies one `float` as `FLD` / `FSTP` does (a signalling NaN comes out quiet).
fn copy_float(e: &mut Engine, from: u32, to: u32) {
    let bits = x87_quiet_bits(e.mem.u32(from));
    e.mem.set_u32(to, bits);
}

// Translated from 00cbcbd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cbcbd0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkLiquidAction::SetTargetObject` (Xbox PDB): with a target, stores it at `+0x10` and
/// looks up the list the target-object map holds for it. If there is none, a two-word
/// list head (8 bytes from the pool, both words cleared) is made and entered under the
/// target (even when the allocation failed, which leaves 0 in the map and returns without
/// adding anything); then this action is added to the list (`00cbc7b0`). The
/// allocation-scope word is 0x10 meanwhile.
pub fn bhk_liquid_action_set_target_object(
    e: &mut Engine,
    this: Ptr<BhkLiquidAction>,
    target: u32,
) {
    let (word, saved) = enter_scope(e);
    if target != 0 {
        e.set(this, BhkLiquidAction::pTargetObject, target);
        let mut list = map_get(e, target);
        if list == 0 {
            let head = e.call(POOL_ALLOCATE, &args![POOL_011F6238, 8u32]).u32();
            if head != 0 {
                e.mem.set_u32(head, 0);
                e.mem.set_u32(head + 4, 0);
                list = head;
            }
            let key = e.get(this, BhkLiquidAction::pTargetObject);
            e.call(MAP_SET_AT, &args![TARGET_MAP, key, list]);
            if list == 0 {
                e.mem.set_u32(word, saved);
                return;
            }
        }
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), this.addr());
            fn_00cbc7b0(e, Ptr::new(list), slot);
        });
    }
    e.mem.set_u32(word, saved);
}

/// Reads a `float` as the x87 does (`FLD`): exact in `f64`.
fn fld(e: &Engine, address: u32) -> f64 {
    f64::from(e.mem.f32(address))
}

// Translated from 00cbccb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cbccb0 - the /GS cookie check is not modeled.
/// `bhkLiquidAction::Update` (Xbox PDB): thiscall with one `float` (the step, `RET 4`).
/// The identities of the objects reached through `+8` are not confirmed: `+8` is the
/// action's Havok object, its `+0x18` a world object, whose `+0xC` is the body wrapper
/// ("holder" below) whose `+8` is the rigid body.
///
/// With the holder present: while no target is set, a target is taken from a property of
/// the holder's Havok object and the player reference (`SetTargetObject`). The force
/// starts as the three words at `011f426c`. The body's linear velocity (`+0x1B0`, or the
/// default vector) is copied. While the stick point (`+0x24`) equals that zero vector
/// the body's contact list is searched: the first contact between this body and another
/// with layer 1 or 0xD whose last point's normal `z` is below 0.5 makes the stick point
/// (and the normal, `+0x30`) and zeroes the velocity. With a stick point the vector
/// from the body to it is projected on the normal and added to the force; the velocity
/// is scaled by `exp(factor * 1/... * step)` (the factor from `011b2180` / `011b2184`,
/// the `double` at `0101a6b0`), gravity (when `0126842c` is set or a stick point exists)
/// is added to `z`, and the velocity is set on the rigid body. The neighbours (the list the
/// target-object map holds for the target) push the force away or toward their velocity
/// within twice the neighbour distance. A target object adds its rotated stick force
/// (the force decays by 0.9 per update). The force, scaled by `01268428`, is written to
/// the Havok object's `+0x20` and the rigid body is activated unless it equals the zero
/// vector. At the end `bFirstUpdate` (`+0x3C`) is cleared whether or not the holder was
/// there.
pub fn bhk_liquid_action_update(e: &mut Engine, this: Ptr<BhkLiquidAction>, step: f32) {
    let me = this.addr();
    let hk_object = if me != 0 { e.mem.u32(me + 8) } else { 0 };
    let world_object = if hk_object != 0 {
        e.mem.u32(hk_object + 0x18)
    } else {
        0
    };
    if world_object != 0 {
        let holder = e.mem.u32(world_object + 0xc);
        if holder != 0 {
            update_holder(e, this, holder, step);
        }
    }
    e.set(this, BhkLiquidAction::bFirstUpdate, 0);
}

/// The part of [`bhk_liquid_action_update`] that runs with a holder. `frame` mirrors the
/// part of the game's stack frame that is passed to callees by address.
fn update_holder(e: &mut Engine, this: Ptr<BhkLiquidAction>, holder: u32, step: f32) {
    // The exe's frame (`AND ESP,0xfffffff0`, `SUB ESP,0xb4`, three pushes): 0xc0 bytes from a
    // 16-byte boundary.
    e.with_stack(0xc0 + 0x10, |e, raw| {
        let frame = (raw.addr() + 0xf) & !0xf;
        update_holder_in_frame(e, this, holder, step, frame);
    });
}

/// [`update_holder`] in the exe's frame (`frame` is its lowest address).
fn update_holder_in_frame(
    e: &mut Engine,
    this: Ptr<BhkLiquidAction>,
    holder: u32,
    step: f32,
    frame: u32,
) {
    let me = this.addr();
    let mut force = [
        f32::from_bits(e.mem.u32(ZERO_VECTOR)),
        f32::from_bits(e.mem.u32(ZERO_VECTOR + 4)),
        f32::from_bits(e.mem.u32(ZERO_VECTOR + 8)),
    ];
    if e.get(this, BhkLiquidAction::pTargetObject) == 0 {
        let hk_object = e.mem.u32(holder + 8);
        // The property holder lands at frame +0x30 (00cbcd28).
        let out = frame + 0x30;
        let found = e
            .call(GET_PROPERTY, &args![out, hk_object, PROPERTY_ID])
            .u32();
        let value = e.mem.u32(found);
        if value != 0 {
            // 00cbcd40: `00653290` (RET 4) is a thiscall on the value (ECX from 00cbcd32).
            let reference = e.call(FIND_BY_KEY, &args![value, FIND_KEY]).u32();
            if reference != 0 {
                let node = e.mem.u32(reference + 8);
                if node != 0 {
                    let target = e.mem.u32(node + 0x18);
                    if target != 0 {
                        bhk_liquid_action_set_target_object(e, this, target);
                    }
                }
            }
        }
    }
    let position = frame + 0x54;
    let velocity = frame + 0x80;
    let first_point = frame + 0x90;
    let second_point = frame + 0xa0;
    let vector = e.vcall(holder, 0xd4, &args![frame + 0x70]).u32();
    e.call(COPY_POINT, &args![position, vector]);
    let body = e.mem.u32(holder + 8);
    let mut gravity_flag = e.mem.u8(GRAVITY_FLAG);
    let mut stick_point_found = false;
    let source = if body != 0 {
        body + 0x1b0
    } else {
        DEFAULT_VECTOR
    };
    let bytes = e.mem.bytes(source, 16);
    e.mem.write(velocity, &bytes);
    if body != 0 && e.call(CONTACT_COUNT, &args![body]).u32() != 0 {
        let stick = me + 0x24;
        if e.call(POINT_EQUAL, &args![stick, ZERO_VECTOR]).bool()
            && e.call(POINT_EQUAL, &args![stick, ZERO_VECTOR]).bool()
        {
            let mut index = 0i32;
            loop {
                let count = e.call(CONTACT_COUNT, &args![body]).i32();
                if index >= count {
                    break;
                }
                let item = e.call(CONTACT_GET, &args![body, index as u32]).u32();
                let owner = e.mem.u32(item + 0xc);
                if e.vcall(owner, 0x20, &args![]).u32() == 0xb {
                    let first = e.mem.u32(item + 0x14);
                    let second = e.mem.u32(item + 0x18);
                    let layer_word = if first == body && second != 0 {
                        Some(e.mem.u32(second + 0x2c))
                    } else if second == body && first != 0 {
                        Some(e.mem.u32(first + 0x2c))
                    } else {
                        None
                    };
                    if let Some(layer_word) = layer_word {
                        let layer = layer_word & 0x7f;
                        let points = e.mem.u32(owner + 0x28);
                        if (layer == 1 || layer == 0xd) && e.mem.u16(points + 4) != 0 {
                            let last = fn_00cbc8f0(e, Ptr::new(owner), 0) as u32 & 0xffff;
                            let data = e.mem.u32(owner + 0xc);
                            let entry = u32::from(e.mem.u8(data + last)) << 5;
                            let source = points + entry + 0x30;
                            let bytes = e.mem.bytes(source, 16);
                            e.mem.write(first_point, &bytes);
                            let bytes = e.mem.bytes(source + 0x10, 16);
                            e.mem.write(second_point, &bytes);
                            if e.global::<f32>(HALF_FLOAT) > e.mem.f32(frame + 0xa8) {
                                e.call(COPY_POINT, &args![stick, first_point]);
                                e.call(POINT_FROM_VECTOR, &args![me + 0x30, second_point]);
                                let zero = e.mem.u32(ZERO_FLOAT);
                                for lane in 0..4 {
                                    e.mem.set_u32(velocity + 4 * lane, zero);
                                }
                            }
                        }
                    }
                }
                index += 1;
                if !e.call(POINT_EQUAL, &args![stick, ZERO_VECTOR]).bool() {
                    break;
                }
            }
        }
    }
    let stick = [
        e.mem.f32(me + 0x24),
        e.mem.f32(me + 0x28),
        e.mem.f32(me + 0x2c),
    ];
    let zero = [
        e.mem.f32(ZERO_VECTOR),
        e.mem.f32(ZERO_VECTOR + 4),
        e.mem.f32(ZERO_VECTOR + 8),
    ];
    let factor_source = if stick[0] == zero[0] && stick[1] == zero[1] && stick[2] == zero[2] {
        FACTOR_FLOAT_RESTING
    } else {
        let to_stick = [
            (f64::from(stick[0]) - fld(e, position)) as f32,
            (f64::from(stick[1]) - fld(e, position + 4)) as f32,
            (f64::from(stick[2]) - fld(e, position + 8)) as f32,
        ];
        gravity_flag = 1;
        stick_point_found = true;
        let (tx, ty, tz) = (
            f64::from(to_stick[0]),
            f64::from(to_stick[1]),
            f64::from(to_stick[2]),
        );
        let squared = ((tx * tx + ty * ty) + tz * tz) as f32;
        let length = e.call(X87_SQRT, &args![f64::from(squared)]).f64() as f32;
        if length > e.global::<f32>(ZERO_FLOAT) {
            let normal = [fld(e, me + 0x30), fld(e, me + 0x34), fld(e, me + 0x38)];
            let along = ((normal[1] * ty + normal[0] * tx) + normal[2] * tz) as f32;
            for lane in 0..3 {
                let part = (normal[lane] * f64::from(along)) as f32;
                force[lane] = (f64::from(force[lane]) + f64::from(part)) as f32;
            }
        }
        FACTOR_FLOAT_MOVING
    };
    let factor_value = x87_quiet(e.mem.f32(factor_source));
    let scaled =
        ((f64::from(factor_value) * e.global::<f64>(SPEED_FACTOR_DOUBLE)) * f64::from(step)) as f32;
    let decay = e.call(X87_EXP, &args![f64::from(scaled)]).f64() as f32;
    let decay = x87_quiet(decay);
    let mut lanes = [0f32; 4];
    for (lane, value) in lanes.iter_mut().enumerate() {
        *value = e.mem.f32(velocity + 4 * lane as u32) * decay;
    }
    if gravity_flag != 0 {
        let zero_bits = e.mem.u32(ZERO_FLOAT);
        let pull = ((-f64::from(step) * fld(e, GRAVITY_SCALE_A)) * fld(e, GRAVITY_SCALE_B)) as f32;
        let pull = x87_quiet(pull);
        let gravity = [
            f32::from_bits(zero_bits),
            f32::from_bits(zero_bits),
            pull,
            f32::from_bits(zero_bits),
        ];
        // ADDPS: gravity (the destination) plus the scaled velocity.
        #[allow(clippy::assign_op_pattern)]
        for lane in 0..4 {
            lanes[lane] = gravity[lane] + lanes[lane];
        }
    }
    for (lane, value) in lanes.iter().enumerate() {
        e.mem.set_f32(velocity + 4 * lane as u32, *value);
    }
    let rigid_body = e.mem.u32(holder + 8);
    if rigid_body != 0 {
        e.call(BRACKET, &args![holder]);
        e.call(SET_LINEAR_VELOCITY, &args![rigid_body, velocity]);
        e.call(BRACKET, &args![holder]);
    }
    let target = e.get(this, BhkLiquidAction::pTargetObject);
    let list = map_get(e, target);
    if list != 0 {
        update_neighbours(e, this, list, frame, stick_point_found, &mut force);
    }
    update_stick_force(e, this, &mut force);
    let scale = fld(e, FORCE_SCALE);
    let result = [
        (f64::from(force[0]) * scale) as f32,
        (f64::from(force[1]) * scale) as f32,
        (f64::from(force[2]) * scale) as f32,
        0.0,
    ];
    let hk_object = e.mem.u32(me + 8);
    for (lane, value) in result.iter().enumerate() {
        e.mem.set_f32(hk_object + 0x20 + 4 * lane as u32, *value);
    }
    if !(force[0] == zero[0] && force[1] == zero[1] && force[2] == zero[2]) {
        let rigid_body = e.mem.u32(holder + 8);
        if rigid_body != 0 {
            e.call(BRACKET, &args![holder]);
            e.call(ACTIVATE, &args![rigid_body]);
            e.call(BRACKET, &args![holder]);
        }
    }
}

/// The neighbour loop of [`bhk_liquid_action_update`]: every other action in `list` that
/// is closer than twice the neighbour distance (and not at the same place) adds a force
/// that follows the sign of the distance, reduced by the part along the normal when a
/// stick point exists.
fn update_neighbours(
    e: &mut Engine,
    this: Ptr<BhkLiquidAction>,
    list: u32,
    frame: u32,
    stick_point_found: bool,
    force: &mut [f32; 3],
) {
    let me = this.addr();
    let position = [
        e.mem.f32(frame + 0x54),
        e.mem.f32(frame + 0x58),
        e.mem.f32(frame + 0x5c),
    ];
    let mut node = list;
    loop {
        if e.mem.u32(node + 4) == 0 && e.mem.u32(node) == 0 {
            break;
        }
        let item = e.mem.u32(node);
        let hk_object = if item != 0 { e.mem.u32(item + 8) } else { 0 };
        let other = if hk_object != 0 {
            e.mem.u32(hk_object + 0x18)
        } else {
            0
        };
        let other_holder = if other != 0 {
            e.mem.u32(other + 0xc)
        } else {
            0
        };
        let vector = e.vcall(other_holder, 0xd4, &args![frame + 0x90]).u32();
        let scale = fld(e, NEIGHBOR_VELOCITY_SCALE);
        let moved = [
            (fld(e, vector) * scale) as f32,
            (fld(e, vector + 4) * scale) as f32,
            (fld(e, vector + 8) * scale) as f32,
        ];
        let away = frame + 0x20;
        for lane in 0..3 {
            let difference = (f64::from(position[lane]) - f64::from(moved[lane])) as f32;
            e.mem.set_f32(away + 4 * lane as u32, difference);
        }
        let length = e.call(UNITIZE_GET_LENGTH, &args![away]).f32();
        let distance = fld(e, me + 0x14);
        if length > 0.0 && distance + distance > f64::from(length) {
            let ratio =
                ((distance - f64::from(length)) / (distance * fld(e, NEIGHBOR_RANGE_SCALE))) as f32;
            let mut ratio = x87_quiet(ratio);
            if f64::from(ratio) > e.global::<f64>(CLAMP_HIGH_DOUBLE) {
                ratio = 1.0;
            }
            if f64::from(ratio) < e.global::<f64>(CLAMP_LOW_DOUBLE) {
                ratio = x87_quiet(e.global::<f32>(NEGATIVE_HALF_FLOAT));
            }
            let strength = (fld(e, me + 0x18) * f64::from(ratio)) as f32;
            let mut pushed = [0f32; 3];
            for lane in 0..3 {
                pushed[lane] = (fld(e, away + 4 * lane as u32) * f64::from(strength)) as f32;
            }
            if stick_point_found {
                let normal = [fld(e, me + 0x30), fld(e, me + 0x34), fld(e, me + 0x38)];
                let along = ((normal[1] * f64::from(pushed[1]) + f64::from(pushed[0]) * normal[0])
                    + normal[2] * f64::from(pushed[2])) as f32;
                for lane in 0..3 {
                    let part = (normal[lane] * f64::from(along)) as f32;
                    pushed[lane] = (f64::from(pushed[lane]) - f64::from(part)) as f32;
                }
            }
            for lane in 0..3 {
                force[lane] = (f64::from(pushed[lane]) + f64::from(force[lane])) as f32;
            }
        }
        node = e.mem.u32(node + 4);
        if node == 0 {
            break;
        }
    }
}

/// The stick-force part of [`bhk_liquid_action_update`]: with a target object, the
/// direction vector at `011a9484` scaled by minus the stick force `+0x1C` is rotated by
/// the target's 3x3 matrix (rows at `+0x68`, `+0x74`, `+0x80`) and added to the force;
/// the stick force then decays (times the `double` at `0106b9e8`).
fn update_stick_force(e: &mut Engine, this: Ptr<BhkLiquidAction>, force: &mut [f32; 3]) {
    let me = this.addr();
    let target = e.get(this, BhkLiquidAction::pTargetObject);
    if target == 0 {
        return;
    }
    let negated = f32::from_bits(x87_quiet_bits(e.mem.u32(me + 0x1c)) ^ 0x8000_0000);
    let direction = [
        fld(e, STICK_DIRECTION),
        fld(e, STICK_DIRECTION + 4),
        fld(e, STICK_DIRECTION + 8),
    ];
    let mut scaled = [0f64; 3];
    for lane in 0..3 {
        scaled[lane] = f64::from((f64::from(negated) * direction[lane]) as f32);
    }
    let matrix = |e: &Engine, row: u32, column: u32| fld(e, target + 0x68 + row * 12 + column * 4);
    let mut rotated = [0f32; 3];
    for (row, value) in rotated.iter_mut().enumerate() {
        let row = row as u32;
        let first = matrix(e, row, 0) * scaled[0] + matrix(e, row, 1) * scaled[1];
        *value = (first + matrix(e, row, 2) * scaled[2]) as f32;
    }
    let decayed = (fld(e, me + 0x1c) * e.global::<f64>(STICK_DECAY_DOUBLE)) as f32;
    e.mem.set_f32(me + 0x1c, decayed);
    for lane in 0..3 {
        force[lane] = (f64::from(rotated[lane]) + f64::from(force[lane])) as f32;
    }
}

// Translated from 00cbd4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<...>::_scalar_deleting_destructor_` (Xbox PDB): the map destructor body
/// (`00cbc740`) and, when bit 0 of `flags` is set, the block is returned to the pool.
/// Answers `this`.
pub fn ni_t_map_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    fn_00cbc740(e, this);
    if flags & 1 != 0 {
        e.call(POOL_FREE, &args![POOL_011F6238, this]);
    }
    this
}

// Translated from 00cbd500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<...>::DeleteItem` (Xbox PDB), vtable slot `0x18` of the map (`this` is not used):
/// clears the value word at `+8` of the map item and tail-calls the pool free (`00aa4060`)
/// on it.
pub fn ni_t_map_delete_item(e: &mut Engine, _this: Ptr, item: u32) {
    e.mem.set_u32(item + 8, 0);
    e.call(POOL_FREE, &args![POOL_011F6238, item]);
}

// Translated from 00cbd520 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cbd520 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkLiquidAction::Init` (Xbox PDB): with the allocation-scope word at 0x10, copies the
/// cinfo's `+8`, `+0xC`, `+0x10`, `+0x14` to `+0x1C`, `+0x20`, `+0x14`, `+0x18`, allocates
/// 0x30 bytes through the router (size word at `+4`), constructs the `hkLiquidAction`
/// there (`00d2c910`, `(info + 4, 0)`), hands it to the action's slot `0x8C`, and
/// releases it. When the property of the cinfo's `+4` object is set, the player
/// reference's target becomes the action's target (`SetTargetObject`). Finally slot
/// `0xBC` is called with the cinfo.
pub fn bhk_liquid_action_init(e: &mut Engine, this: Ptr<BhkLiquidAction>, info: u32) {
    let me = this.addr();
    let (word, saved) = enter_scope(e);
    copy_float(e, info + 8, me + 0x1c);
    copy_float(e, info + 0xc, me + 0x20);
    copy_float(e, info + 0x10, me + 0x14);
    copy_float(e, info + 0x14, me + 0x18);
    let block = allocate_sized(e, 0x30);
    let hk_object = e.mem.u32(info + 4);
    let action = e
        .call(HK_LIQUID_ACTION_CONSTRUCT, &args![block, hk_object, 0u32])
        .u32();
    e.vcall(me, 0x8c, &args![action]);
    e.call(REMOVE_REFERENCE, &args![action]);
    let hk_object = e.mem.u32(info + 4);
    // The `hkpPropertyValue` local `00c85ef0` returns by value (8 bytes).
    let value = e.with_stack(8, |e, out| {
        let found = e
            .call(GET_PROPERTY, &args![out, hk_object, PROPERTY_ID])
            .u32();
        e.mem.u32(found)
    });
    if value != 0 {
        // 00cbd5fc: `00653290` (RET 4) is a thiscall on the value (ECX from 00cbd5ee).
        let reference = e.call(FIND_BY_KEY, &args![value, FIND_KEY]).u32();
        if reference != 0 {
            let node = e.mem.u32(reference + 8);
            if node != 0 {
                let target = e.mem.u32(node + 0x18);
                if target != 0 {
                    bhk_liquid_action_set_target_object(e, this, target);
                }
            }
        }
    }
    e.vcall(me, 0xbc, &args![info]);
    e.mem.set_u32(word, saved);
}

// Translated from 00cbd650 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cbd650 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkLiquidAction::GetViewerStrings` (Xbox PDB): with the allocation-scope word at
/// 0x10, the `bhkUnaryAction` strings (`00c86b80`), then the viewer string of the class
/// `NiRTTI` (the global at `01268420`).
pub fn bhk_liquid_action_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<BhkLiquidAction>,
    strings: u32,
) {
    let (word, saved) = enter_scope(e);
    e.call(UNARY_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(ACTION_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    push_viewer_string(e, strings, text);
    e.mem.set_u32(word, saved);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00cbc650,
            bhk_liquid_action_load_binary(Ptr<BhkLiquidAction>, u32)
        ),
        entry!(
            0x00cbc6a0,
            bhk_liquid_action_save_binary(Ptr<BhkLiquidAction>, u32)
        ),
        entry!(0x00cbc720, fn_00cbc720(Ptr)),
        entry!(0x00cbc740, fn_00cbc740(Ptr)),
        entry!(0x00cbc7b0, fn_00cbc7b0(Ptr, Ptr)),
        entry!(0x00cbc810, fn_00cbc810(Ptr, Ptr)),
        entry!(0x00cbc8b0, ni_t_map_base_scalar_deleting_destructor(Ptr, u8) -> Ptr),
        entry!(0x00cbc8f0, fn_00cbc8f0(Ptr, u32) -> i32),
        entry!(0x00cbc920, bhk_liquid_action_destruct(Ptr<BhkLiquidAction>)),
        entry!(
            0x00cbca20,
            bhk_liquid_action_create_clone(Ptr<BhkLiquidAction>, u32) -> u32
        ),
        entry!(
            0x00cbcad0,
            bhk_liquid_action_create_cinfo(Ptr<BhkLiquidAction>, Ptr) -> u32
        ),
        entry!(
            0x00cbcbd0,
            bhk_liquid_action_set_target_object(Ptr<BhkLiquidAction>, u32)
        ),
        entry!(
            0x00cbccb0,
            bhk_liquid_action_update(Ptr<BhkLiquidAction>, f32)
        ),
        entry!(0x00cbd4e0, ni_t_map_scalar_deleting_destructor(Ptr, u8) -> Ptr),
        entry!(0x00cbd500, ni_t_map_delete_item(Ptr, u32)),
        entry!(
            0x00cbd520,
            bhk_liquid_action_init(Ptr<BhkLiquidAction>, u32)
        ),
        entry!(
            0x00cbd650,
            bhk_liquid_action_get_viewer_strings(Ptr<BhkLiquidAction>, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::super::bhklimitedhingeconstraint::support::*;
    use super::*;
    use std::rc::Rc;

    const SCOPE: u32 = 0x2b4;

    fn list_00cbc7b0(e: &mut Engine, head: u32, link: u32, source: u32) -> (Ptr, Ptr) {
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, head);
        e.mem.set_u32(this + 4, link);
        let word = e.mem.alloc(4);
        e.mem.set_u32(word, source);
        (Ptr::<()>::new(this), Ptr::<()>::new(word))
    }

    #[test]
    fn fn_00cbc7b0_ignores_a_null_word_and_fills_an_empty_head() {
        let mut e = Engine::new();
        let (this, source) = list_00cbc7b0(&mut e, 0, 0x77, 0);
        e.call(0x00cb_c7b0, &args![this, source]);
        assert_eq!(e.mem.u32(this.addr()), 0);
        let (this, source) = list_00cbc7b0(&mut e, 0, 0x77, 0x55);
        e.call_log = Some(vec![]);
        e.call(0x00cb_c7b0, &args![this, source]);
        assert_eq!(e.mem.u32(this.addr()), 0x55);
        assert_eq!(e.mem.u32(this.addr() + 4), 0x77);
        assert!(e
            .call_log
            .take()
            .unwrap()
            .iter()
            .all(|(a, _)| *a != POOL_ALLOCATE));
    }

    #[test]
    fn fn_00cbc7b0_pushes_the_old_head_on_a_new_node() {
        let mut e = Engine::new();
        let node = e.mem.alloc(8);
        e.register_double(POOL_ALLOCATE, move |_, a| {
            assert_eq!(a, [POOL_011F6238, 8]);
            Ret {
                eax: node,
                ..Ret::default()
            }
        });
        let (this, source) = list_00cbc7b0(&mut e, 0x11, 0x77, 0x55);
        e.call(0x00cb_c7b0, &args![this, source]);
        assert_eq!(e.mem.u32(node), 0x11);
        assert_eq!(e.mem.u32(node + 4), 0x77);
        assert_eq!(e.mem.u32(this.addr()), 0x55);
        assert_eq!(e.mem.u32(this.addr() + 4), node);
    }

    /// An action whose slot `0xB4` answers `cinfo` (four words at `+8..+0x14`).
    fn action_with_cinfo(e: &mut Engine, cinfo: u32) -> u32 {
        let this = e.mem.alloc(0x40);
        let table = e.mem.alloc(0xc0);
        e.mem.set_u32(this, table);
        e.mem.set_u32(table + 0xb4, 0x0900_00b4);
        e.register_double(0x0900_00b4, move |e, a| {
            e.mem.set_u8(a[1], 1);
            ret(cinfo)
        });
        this
    }

    #[test]
    fn load_and_save_binary_swap_the_cinfo_words_only_for_a_foreign_stream() {
        let mut e = Engine::new();
        let log: Calls = Rc::default();
        record(&mut e, UNARY_LOAD_BINARY, 0, &log);
        record(&mut e, UNARY_SAVE_BINARY, 0, &log);
        let cinfo = e.mem.alloc(0x20);
        let this = action_with_cinfo(&mut e, cinfo);
        let stream = e.mem.alloc(0x300);
        for offset in [8, 0xc, 0x10, 0x14] {
            e.mem.set_u32(cinfo + offset, 0x0102_0304);
        }
        // PC byte order: untouched.
        e.mem.set_u8(stream + 0x1e5, 1);
        e.call(0x00cb_c650, &args![this, stream]);
        assert_eq!(e.mem.u32(cinfo + 8), 0x0102_0304);
        assert_eq!(calls_to(&log, UNARY_LOAD_BINARY), [[this, stream]]);
        // Foreign order: swapped after the base load.
        e.mem.set_u8(stream + 0x1e5, 0);
        e.call(0x00cb_c650, &args![this, stream]);
        for offset in [8, 0xc, 0x10, 0x14] {
            assert_eq!(e.mem.u32(cinfo + offset), 0x0403_0201);
        }
        assert_eq!(e.mem.u32(cinfo + 0x18), 0);
        // Save: swapped for the base save and swapped back.
        e.mem.set_u8(stream + 0x1e4, 0);
        e.register_double(UNARY_SAVE_BINARY, {
            move |e, _| {
                assert_eq!(e.mem.u32(cinfo + 8), 0x0102_0304);
                Ret::default()
            }
        });
        e.call(0x00cb_c6a0, &args![this, stream]);
        assert_eq!(e.mem.u32(cinfo + 8), 0x0403_0201);
        e.mem.set_u8(stream + 0x1e4, 1);
        e.register_double(UNARY_SAVE_BINARY, move |e, _| {
            assert_eq!(e.mem.u32(cinfo + 8), 0x0403_0201);
            Ret::default()
        });
        e.call(0x00cb_c6a0, &args![this, stream]);
        assert_eq!(e.mem.u32(cinfo + 8), 0x0403_0201);
    }

    #[test]
    fn map_destructors_set_their_vtables_remove_everything_and_free_the_buckets() {
        let mut e = Engine::new();
        let log: Calls = Rc::default();
        record(&mut e, MAP_REMOVE_ALL, 0, &log);
        record(&mut e, FREE, 0, &log);
        record(&mut e, POOL_FREE, 0, &log);
        let map = e.mem.alloc(0x14);
        e.mem.set_u32(map + 8, 0x7000);
        e.call(0x00cb_c720, &args![map]);
        assert_eq!(e.mem.u32(map), VTABLE_TI_MAP_BASE);
        assert_eq!(calls_to(&log, MAP_REMOVE_ALL), [[map]]);
        assert_eq!(calls_to(&log, FREE), [[0x7000]]);
        // The map itself removes twice, the second time as the base.
        e.register_double(MAP_REMOVE_ALL, {
            let seen = Rc::new(std::cell::RefCell::new(Vec::new()));
            move |e, a| {
                seen.borrow_mut().push(e.mem.u32(a[0]));
                if seen.borrow().len() == 2 {
                    assert_eq!(*seen.borrow(), [VTABLE_TI_MAP, VTABLE_TI_MAP_BASE]);
                }
                Ret::default()
            }
        });
        e.call(0x00cb_c740, &args![map]);
        assert_eq!(e.mem.u32(map), VTABLE_TI_MAP_BASE);
        // Scalar deleting forms free the block only when bit 0 is set and answer `this`.
        assert_eq!(e.call(0x00cb_c8b0, &args![map, 0xfeu32]).u32(), map);
        assert!(calls_to(&log, POOL_FREE).is_empty());
        assert_eq!(e.call(0x00cb_c8b0, &args![map, 1u32]).u32(), map);
        assert_eq!(calls_to(&log, POOL_FREE), [[POOL_011F6238, map]]);
        assert_eq!(e.call(0x00cb_d4e0, &args![map, 0u32]).u32(), map);
        assert_eq!(e.call(0x00cb_d4e0, &args![map, 3u32]).u32(), map);
        assert_eq!(calls_to(&log, POOL_FREE).len(), 2);
        assert_eq!(e.mem.u32(map), VTABLE_TI_MAP_BASE);
    }

    #[test]
    fn delete_item_clears_the_value_and_frees_the_item() {
        let mut e = Engine::new();
        let log: Calls = Rc::default();
        record(&mut e, POOL_FREE, 0, &log);
        let item = e.mem.alloc(0x10);
        e.mem.set_u32(item + 8, 0x1234);
        e.call(0x00cb_d500, &args![0u32, item]);
        assert_eq!(e.mem.u32(item + 8), 0);
        assert_eq!(calls_to(&log, POOL_FREE), [[POOL_011F6238, item]]);
    }

    /// A three-node list: values `a`, `b`, `c`; returns the node addresses.
    fn three_nodes(e: &mut Engine, a: u32, b: u32, c: u32) -> [u32; 3] {
        let nodes = [e.mem.alloc(8), e.mem.alloc(8), e.mem.alloc(8)];
        for (i, value) in [a, b, c].into_iter().enumerate() {
            e.mem.set_u32(nodes[i], value);
            e.mem
                .set_u32(nodes[i] + 4, nodes.get(i + 1).copied().unwrap_or(0));
        }
        nodes
    }

    fn remove(e: &mut Engine, head: u32, value: u32) {
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, value);
        e.call(0x00cb_c810, &args![head, slot]);
    }

    #[test]
    fn list_remove_unlinks_the_middle_node_then_the_head_then_empties_the_list() {
        let mut e = Engine::new();
        let log: Calls = Rc::default();
        record(&mut e, LIST_CLEAR, 0, &log);
        record(&mut e, POOL_FREE, 0, &log);
        let [head, second, third] = three_nodes(&mut e, 0xa, 0xb, 0xc);
        // Not found, null value: nothing changes.
        remove(&mut e, head, 0xd);
        remove(&mut e, head, 0);
        assert_eq!(e.mem.u32(head + 4), second);
        assert!(log.borrow().is_empty());
        // Middle node.
        remove(&mut e, head, 0xb);
        assert_eq!(e.mem.u32(head + 4), third);
        assert_eq!(e.mem.u32(second + 4), 0);
        assert_eq!(calls_to(&log, LIST_CLEAR), [[second]]);
        assert_eq!(calls_to(&log, POOL_FREE), [[POOL_011F6238, second]]);
        // Head: the next node's value and link move up and that node is released.
        remove(&mut e, head, 0xa);
        assert_eq!((e.mem.u32(head), e.mem.u32(head + 4)), (0xc, 0));
        assert_eq!(calls_to(&log, LIST_CLEAR)[1], [third]);
        assert_eq!(calls_to(&log, POOL_FREE)[1], [POOL_011F6238, third]);
        // The head alone just becomes empty; an empty list is left alone.
        remove(&mut e, head, 0xc);
        assert_eq!((e.mem.u32(head), e.mem.u32(head + 4)), (0, 0));
        remove(&mut e, head, 0xc);
        assert_eq!(log.borrow().len(), 4);
    }

    #[test]
    fn last_index_of_a_byte_walks_down_from_the_end() {
        let mut e = Engine::new();
        let data = e.mem.alloc(8);
        for (i, byte) in [5u8, 7, 5, 9].into_iter().enumerate() {
            e.mem.set_u8(data + i as u32, byte);
        }
        let array = e.mem.alloc(0x14);
        e.mem.set_u32(array + 0xc, data);
        e.mem.set_u32(array + 0x10, 4);
        assert_eq!(e.call(0x00cb_c8f0, &args![array, 5u32]).i32(), 2);
        assert_eq!(e.call(0x00cb_c8f0, &args![array, 9u32]).i32(), 3);
        assert_eq!(e.call(0x00cb_c8f0, &args![array, 7u32]).i32(), 1);
        assert_eq!(e.call(0x00cb_c8f0, &args![array, 1u32]).i32(), -1);
        e.mem.set_u32(array + 0x10, 0);
        assert_eq!(e.call(0x00cb_c8f0, &args![array, 5u32]).i32(), -1);
    }

    /// An engine with the counters and the pool/map doubles' memory mapped.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e
    }

    #[test]
    fn destruct_takes_the_action_out_of_the_target_list_and_drops_an_emptied_entry() {
        let mut e = engine();
        e.set_global(COUNT_LIQUID_ACTION, 3u32);
        let log: Calls = Rc::default();
        record(&mut e, SERIALIZABLE_KILL, 0, &log);
        record(&mut e, UNARY_DESTRUCT, 0, &log);
        record(&mut e, MAP_REMOVE_AT, 0, &log);
        record(&mut e, LIST_CLEAR, 0, &log);
        record(&mut e, POOL_FREE, 0, &log);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x10, 0x4444);
        let list = e.mem.alloc(8);
        e.mem.set_u32(list, this);
        e.register_double(MAP_GET_AT, move |e, a| {
            assert_eq!(a[..2], [TARGET_MAP, 0x4444]);
            e.mem.set_u32(a[2], list);
            ret(1)
        });
        let word = e.tls() + SCOPE;
        e.mem.set_u32(word, 0x77);
        e.call(0x00cb_c920, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_LIQUID_ACTION);
        assert_eq!(e.mem.u32(list), 0);
        assert_eq!(calls_to(&log, MAP_REMOVE_AT), [[TARGET_MAP, 0x4444]]);
        assert_eq!(calls_to(&log, LIST_CLEAR), [[list]]);
        assert_eq!(calls_to(&log, POOL_FREE), [[POOL_011F6238, list]]);
        assert_eq!(calls_to(&log, SERIALIZABLE_KILL), [[this]]);
        assert_eq!(calls_to(&log, UNARY_DESTRUCT), [[this]]);
        assert_eq!(e.global::<u32>(COUNT_LIQUID_ACTION), 2);
        assert_eq!(e.mem.u32(word), 0x77);
        // Another action stays in the list: the entry is kept. No target: nothing at all.
        let other = e.mem.alloc(8);
        let list2 = e.mem.alloc(8);
        e.mem.set_u32(list2, this);
        e.mem.set_u32(list2 + 4, other);
        e.mem.set_u32(other, 0x99);
        e.register_double(MAP_GET_AT, move |e, a| {
            e.mem.set_u32(a[2], list2);
            ret(1)
        });
        e.call(0x00cb_c920, &args![this]);
        assert_eq!(calls_to(&log, MAP_REMOVE_AT).len(), 1);
        assert_eq!(e.mem.u32(list2), 0x99);
        e.mem.set_u32(this + 0x10, 0);
        let before = log.borrow().len();
        e.call(0x00cb_c920, &args![this]);
        assert_eq!(log.borrow().len(), before + 2);
    }

    #[test]
    fn create_clone_constructs_and_copies_inside_the_clone_lock() {
        let mut e = engine();
        let log: Calls = Rc::default();
        record(&mut e, CLONE_CRIT_LOCK, 0, &log);
        record(&mut e, CLONE_CRIT_UNLOCK, 0, &log);
        record(&mut e, ALLOCATE, 0x6000, &log);
        record(&mut e, CONSTRUCT, 0x6001, &log);
        record(&mut e, UNARY_COPY_MEMBERS, 0, &log);
        let word = e.tls() + SCOPE;
        e.mem.set_u32(word, 0x77);
        assert_eq!(
            e.call(0x00cb_ca20, &args![0x5000u32, 0x99u32]).u32(),
            0x6001
        );
        let order: Vec<u32> = log.borrow().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            [
                CLONE_CRIT_LOCK,
                ALLOCATE,
                CONSTRUCT,
                UNARY_COPY_MEMBERS,
                CLONE_CRIT_UNLOCK
            ]
        );
        assert_eq!(calls_to(&log, ALLOCATE), [[0x40]]);
        assert_eq!(calls_to(&log, CONSTRUCT), [[0x6000]]);
        assert_eq!(calls_to(&log, UNARY_COPY_MEMBERS), [[0x5000, 0x6001, 0x99]]);
        assert_eq!(e.mem.u32(word), 0x77);
        // A failed allocation copies into nothing and answers 0.
        record(&mut e, ALLOCATE, 0, &log);
        assert_eq!(e.call(0x00cb_ca20, &args![0x5000u32, 0x98u32]).u32(), 0);
        assert_eq!(calls_to(&log, CONSTRUCT).len(), 1);
        assert_eq!(calls_to(&log, UNARY_COPY_MEMBERS)[1], [0x5000, 0, 0x98]);
    }

    #[test]
    fn create_cinfo_aligns_the_block_fills_it_and_copies_from_the_action() {
        let mut e = engine();
        e.set_global(0x0103_0020, 1.0f32);
        e.set_global(0x0101_e580, 2.0f32);
        e.set_global(0x0101_e6ec, 3.0f32);
        let block = e.mem.alloc(0x40) + 4;
        let log: Calls = Rc::default();
        record(&mut e, POOL_ALLOCATE, block, &log);
        record(&mut e, UNARY_COPY_CINFO, 0, &log);
        let this = e.mem.alloc(0x40);
        e.mem.set_f32(this + 0x14, 10.0);
        e.mem.set_f32(this + 0x18, 11.0);
        e.mem.set_f32(this + 0x1c, 12.0);
        e.mem.set_f32(this + 0x20, 13.0);
        let created = e.mem.alloc(4);
        let word = e.tls() + SCOPE;
        e.mem.set_u32(word, 0x77);
        let info = e.call(0x00cb_cad0, &args![this, created]).u32();
        let pad = 0x10 - (block & 0xf);
        assert_eq!(info, block + pad);
        assert_eq!(e.mem.u8(info - 1), pad as u8);
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(calls_to(&log, POOL_ALLOCATE), [[POOL_011F6238, 0x28]]);
        assert_eq!(e.mem.u32(this + 0xc), info);
        assert_eq!(
            [
                e.mem.f32(info + 8),
                e.mem.f32(info + 0xc),
                e.mem.f32(info + 0x10),
                e.mem.f32(info + 0x14)
            ],
            [1.0, 2.0, 3.0, 2.0]
        );
        // No Havok object: CopyCinfo is not called and the defaults stay.
        assert!(calls_to(&log, UNARY_COPY_CINFO).is_empty());
        assert_eq!(e.mem.u32(word), 0x77);
        // Second time it is kept; with a Havok object a fresh one copies the action's values.
        assert_eq!(e.call(0x00cb_cad0, &args![this, created]).u32(), info);
        assert_eq!(e.mem.u8(created), 0);
        e.mem.set_u32(this + 0xc, 0);
        e.mem.set_u32(this + 8, 0x5000);
        e.call(0x00cb_cad0, &args![this, created]);
        assert_eq!(calls_to(&log, UNARY_COPY_CINFO), [[this, info]]);
        assert_eq!(
            [
                e.mem.f32(info + 8),
                e.mem.f32(info + 0xc),
                e.mem.f32(info + 0x10),
                e.mem.f32(info + 0x14)
            ],
            [12.0, 13.0, 10.0, 11.0]
        );
    }

    #[test]
    fn set_target_object_makes_the_list_once_and_adds_the_action() {
        let mut e = engine();
        let log: Calls = Rc::default();
        let head = e.mem.alloc(8);
        record(&mut e, POOL_ALLOCATE, head, &log);
        record(&mut e, MAP_SET_AT, 0, &log);
        let found = Rc::new(std::cell::Cell::new(0u32));
        let lookup = Rc::clone(&found);
        e.register_double(MAP_GET_AT, move |e, a| {
            e.mem.set_u32(a[2], lookup.get());
            ret(1)
        });
        let this = e.mem.alloc(0x40);
        let word = e.tls() + SCOPE;
        e.mem.set_u32(word, 0x77);
        e.mem.set_u32(head, 0x1111);
        e.call(0x00cb_cbd0, &args![this, 0x4444u32]);
        assert_eq!(e.mem.u32(this + 0x10), 0x4444);
        assert_eq!(calls_to(&log, POOL_ALLOCATE), [[POOL_011F6238, 8]]);
        assert_eq!(calls_to(&log, MAP_SET_AT), [[TARGET_MAP, 0x4444, head]]);
        // The new head took the action as its first value.
        assert_eq!(e.mem.u32(head), this);
        assert_eq!(e.mem.u32(word), 0x77);
        // An existing list is reused: the action is pushed on it.
        let list = e.mem.alloc(8);
        e.mem.set_u32(list + 4, 0x2222);
        found.set(list);
        e.call(0x00cb_cbd0, &args![this, 0x4444u32]);
        assert_eq!(calls_to(&log, POOL_ALLOCATE).len(), 1);
        assert_eq!(e.mem.u32(list), this);
        // No target: nothing; a failed pool allocation maps the target to nothing.
        let before = log.borrow().len();
        e.call(0x00cb_cbd0, &args![this, 0u32]);
        assert_eq!(log.borrow().len(), before);
        found.set(0);
        record(&mut e, POOL_ALLOCATE, 0, &log);
        e.call(0x00cb_cbd0, &args![this, 0x5555u32]);
        assert_eq!(calls_to(&log, MAP_SET_AT)[1], [TARGET_MAP, 0x5555, 0]);
        assert_eq!(e.mem.u32(word), 0x77);
    }

    #[test]
    fn init_copies_the_cinfo_builds_the_havok_action_and_picks_up_the_player_target() {
        let (mut e, word, block) = engine_with_allocator();
        e.map(0x0100_0000, 0x0030_0000);
        let log: Calls = Rc::default();
        record(&mut e, HK_LIQUID_ACTION_CONSTRUCT, 0x6000, &log);
        record(&mut e, REMOVE_REFERENCE, 0, &log);
        record(&mut e, FIND_BY_KEY, 0, &log);
        let property = e.mem.alloc(4);
        record(&mut e, GET_PROPERTY, property, &log);
        e.register_double(MAP_GET_AT, |_, _| ret(0));
        let this = e.mem.alloc(0x40);
        let table = e.mem.alloc(0xd0);
        e.mem.set_u32(this, table);
        e.mem.set_u32(table + 0x8c, 0x0900_008c);
        e.mem.set_u32(table + 0xbc, 0x0900_00bc);
        record(&mut e, 0x0900_008c, 0, &log);
        record(&mut e, 0x0900_00bc, 0, &log);
        let info = e.mem.alloc(0x20);
        for (i, value) in [1.0f32, 2.0, 3.0, 4.0].into_iter().enumerate() {
            e.mem.set_f32(info + 8 + 4 * i as u32, value);
        }
        e.mem.set_u32(info + 4, 0x7000);
        e.call(0x00cb_d520, &args![this, info]);
        assert_eq!(
            [
                e.mem.f32(this + 0x1c),
                e.mem.f32(this + 0x20),
                e.mem.f32(this + 0x14),
                e.mem.f32(this + 0x18)
            ],
            [1.0, 2.0, 3.0, 4.0]
        );
        assert_eq!(e.mem.u16(block + 4), 0x30);
        assert_eq!(
            calls_to(&log, HK_LIQUID_ACTION_CONSTRUCT),
            [[block, 0x7000, 0]]
        );
        assert_eq!(calls_to(&log, 0x0900_008c), [[this, 0x6000]]);
        assert_eq!(calls_to(&log, REMOVE_REFERENCE), [[0x6000]]);
        assert_eq!(calls_to(&log, 0x0900_00bc), [[this, info]]);
        // The property is unset: the player reference is not looked up.
        assert!(calls_to(&log, FIND_BY_KEY).is_empty());
        assert_eq!(e.mem.u32(word), 0x77);
        // Property set, player reference present with a target: it becomes the target.
        e.mem.set_u32(property, 1);
        let reference = e.mem.alloc(0x10);
        let node = e.mem.alloc(0x20);
        e.mem.set_u32(reference + 8, node);
        e.mem.set_u32(node + 0x18, 0x3333);
        record(&mut e, FIND_BY_KEY, reference, &log);
        record(&mut e, MAP_SET_AT, 0, &log);
        record(&mut e, POOL_ALLOCATE, 0, &log);
        e.call(0x00cb_d520, &args![this, info]);
        assert_eq!(calls_to(&log, FIND_BY_KEY), [[1, FIND_KEY]]);
        assert_eq!(e.mem.u32(this + 0x10), 0x3333);
        assert_eq!(e.mem.u32(word), 0x77);
    }

    #[test]
    fn get_viewer_strings_appends_the_class_string() {
        let (mut e, word, _) = engine_with_allocator();
        e.map(0x0126_8000, 0x1000);
        e.set_global(ACTION_RTTI, 0x1234u32);
        let log: Calls = Rc::default();
        record(&mut e, UNARY_GET_VIEWER_STRINGS, 0, &log);
        record(&mut e, NI_GET_VIEWER_STRING, 0x3000, &log);
        record(&mut e, 0x0096_ae90, 0, &log);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 4);
        e.call(0x00cb_d650, &args![0x5000u32, strings]);
        assert_eq!(
            calls_to(&log, UNARY_GET_VIEWER_STRINGS),
            [[0x5000, strings]]
        );
        assert_eq!(calls_to(&log, NI_GET_VIEWER_STRING), [[0x1234]]);
        let set = calls_to(&log, 0x0096_ae90);
        assert_eq!((set[0][0], set[0][1]), (strings, 0));
        assert_eq!(e.mem.u32(word), 0x77);
    }

    // ----- Update -----

    use std::cell::RefCell;

    /// The pieces of a world for `Update`: an action, its Havok object chain, a holder
    /// with a rigid body, and doubles for everything outside this file.
    struct Scene {
        e: Engine,
        log: Calls,
        this: u32,
        holder: u32,
        body: u32,
        velocity: Rc<RefCell<Vec<u8>>>,
    }

    /// A holder whose slot `0xD4` (the double at `slot_double`) answers `vector`.
    fn holder_with_vector(e: &mut Engine, slot_double: u32, vector: [f32; 3]) -> u32 {
        let holder = e.mem.alloc(0x20);
        let table = e.mem.alloc(0xe0);
        e.mem.set_u32(holder, table);
        e.mem.set_u32(table + 0xd4, slot_double);
        let block = e.mem.alloc(16);
        for (lane, value) in vector.into_iter().enumerate() {
            e.mem.set_f32(block + 4 * lane as u32, value);
        }
        e.register_double(slot_double, move |_, _| ret(block));
        holder
    }

    /// An action object whose Havok object leads to `holder`.
    fn action_for(e: &mut Engine, holder: u32) -> u32 {
        let this = e.mem.alloc(0x40);
        let hk_object = e.mem.alloc(0x40);
        let world_object = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, hk_object);
        e.mem.set_u32(hk_object + 0x18, world_object);
        e.mem.set_u32(world_object + 0xc, holder);
        this
    }

    fn scene(body_velocity: [f32; 3]) -> Scene {
        let mut e = engine();
        let log: Calls = Rc::default();
        e.set_global(HALF_FLOAT, 0.5f32);
        e.set_global(NEGATIVE_HALF_FLOAT, -0.5f32);
        e.set_global(CLAMP_HIGH_DOUBLE, 1.0f64);
        e.set_global(CLAMP_LOW_DOUBLE, -0.5f64);
        e.set_global(SPEED_FACTOR_DOUBLE, -1.0f64);
        e.set_global(STICK_DECAY_DOUBLE, 0.9f64);
        e.set_global(FACTOR_FLOAT_MOVING, 1.0f32);
        e.set_global(FACTOR_FLOAT_RESTING, 0.5f32);
        e.set_global(GRAVITY_SCALE_A, 2.0f32);
        e.set_global(GRAVITY_SCALE_B, 3.0f32);
        e.set_global(NEIGHBOR_VELOCITY_SCALE, 2.0f32);
        e.set_global(NEIGHBOR_RANGE_SCALE, 0.5f32);
        e.set_global(FORCE_SCALE, 2.0f32);
        e.set_global(STICK_DIRECTION + 8, 1.0f32);
        let holder = holder_with_vector(&mut e, 0x0900_00d4, [0.0, 0.0, 0.0]);
        let this = action_for(&mut e, holder);
        let body = e.mem.alloc(0x200);
        e.mem.set_u32(holder + 8, body);
        for (lane, value) in body_velocity.into_iter().enumerate() {
            e.mem.set_f32(body + 0x1b0 + 4 * lane as u32, value);
        }
        let property = e.mem.alloc(4);
        record(&mut e, GET_PROPERTY, property, &log);
        record(&mut e, CONTACT_COUNT, 0, &log);
        record(&mut e, BRACKET, 0, &log);
        record(&mut e, ACTIVATE, 0, &log);
        record(&mut e, MAP_GET_AT, 0, &log);
        e.register_double(COPY_POINT, |e, a| {
            let bytes = e.mem.bytes(a[1], 12);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        e.register_double(POINT_FROM_VECTOR, |e, a| {
            let bytes = e.mem.bytes(a[1], 12);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        e.register_double(POINT_EQUAL, |e, a| {
            ret(u32::from((0..3).all(|lane| {
                e.mem.f32(a[0] + 4 * lane) == e.mem.f32(a[1] + 4 * lane)
            })))
        });
        e.register_double(X87_EXP, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            Ret {
                st0: value.exp(),
                ..Ret::default()
            }
        });
        e.register_double(UNITIZE_GET_LENGTH, |e, a| {
            let v: Vec<f32> = (0..3).map(|lane| e.mem.f32(a[0] + 4 * lane)).collect();
            let length = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
            for (lane, value) in v.iter().enumerate() {
                e.mem.set_f32(a[0] + 4 * lane as u32, value / length);
            }
            Ret {
                st0: f64::from(length),
                ..Ret::default()
            }
        });
        let velocity = Rc::new(RefCell::new(Vec::new()));
        let captured = Rc::clone(&velocity);
        e.register_double(SET_LINEAR_VELOCITY, move |e, a| {
            *captured.borrow_mut() = e.mem.bytes(a[1], 16);
            Ret::default()
        });
        Scene {
            e,
            log,
            this,
            holder,
            body,
            velocity,
        }
    }

    fn lanes(bytes: &[u8]) -> [f32; 4] {
        let mut out = [0f32; 4];
        for (lane, value) in out.iter_mut().enumerate() {
            *value = f32::from_le_bytes(bytes[4 * lane..4 * lane + 4].try_into().unwrap());
        }
        out
    }

    #[test]
    fn update_without_a_holder_only_clears_the_first_update_flag() {
        let mut s = scene([0.0; 3]);
        s.e.mem.set_u8(s.this + 0x3c, 1);
        s.e.mem.set_u32(s.this + 8, 0);
        s.e.call(0x00cb_ccb0, &args![s.this, 0.5f32]);
        assert_eq!(s.e.mem.u8(s.this + 0x3c), 0);
        assert!(s.log.borrow().is_empty());
        // A world object without a holder: same.
        let hk = s.e.mem.alloc(0x40);
        let world = s.e.mem.alloc(0x20);
        s.e.mem.set_u32(hk + 0x18, world);
        s.e.mem.set_u32(s.this + 8, hk);
        s.e.mem.set_u8(s.this + 0x3c, 1);
        s.e.call(0x00cb_ccb0, &args![s.this, 0.5f32]);
        assert_eq!(s.e.mem.u8(s.this + 0x3c), 0);
        assert!(s.log.borrow().is_empty());
    }

    #[test]
    fn update_at_rest_scales_the_velocity_and_writes_a_zero_force() {
        let mut s = scene([1.0, 2.0, 3.0]);
        s.e.mem.set_u8(s.this + 0x3c, 1);
        s.e.call(0x00cb_ccb0, &args![s.this, 0.5f32]);
        let decay = (-0.25f64).exp() as f32;
        assert_eq!(
            lanes(&s.velocity.borrow()),
            [1.0 * decay, 2.0 * decay, 3.0 * decay, 0.0]
        );
        let hk_object = s.e.mem.u32(s.this + 8);
        assert_eq!(s.e.mem.bytes(hk_object + 0x20, 16), vec![0; 16]);
        // The velocity is set between two brackets and no force means no activation.
        assert_eq!(calls_to(&s.log, BRACKET), [[s.holder], [s.holder]]);
        assert!(calls_to(&s.log, ACTIVATE).is_empty());
        assert_eq!(s.e.mem.u8(s.this + 0x3c), 0);
        // Without a rigid body nothing is set on one.
        s.e.mem.set_u32(s.holder + 8, 0);
        s.e.call(0x00cb_ccb0, &args![s.this, 0.5f32]);
        assert_eq!(calls_to(&s.log, BRACKET).len(), 2);
    }

    #[test]
    fn update_adds_gravity_and_the_rotated_stick_force_and_activates() {
        let mut s = scene([1.0, 2.0, 3.0]);
        s.e.mem.set_u8(GRAVITY_FLAG, 1);
        let target = s.e.mem.alloc(0x100);
        for row in 0..3u32 {
            s.e.mem.set_f32(target + 0x68 + row * 12 + row * 4, 1.0);
        }
        s.e.mem.set_u32(s.this + 0x10, target);
        s.e.mem.set_f32(s.this + 0x1c, 2.0);
        s.e.call(0x00cb_ccb0, &args![s.this, 0.5f32]);
        let decay = (-0.25f64).exp() as f32;
        assert_eq!(
            lanes(&s.velocity.borrow()),
            [1.0 * decay, 2.0 * decay, 3.0 * decay - 3.0, 0.0]
        );
        // The stick force is negated, rotated, added, and decays by 0.9.
        let hk_object = s.e.mem.u32(s.this + 8);
        assert_eq!(
            lanes(&s.e.mem.bytes(hk_object + 0x20, 16)),
            [0.0, 0.0, -4.0, 0.0]
        );
        assert_eq!(s.e.mem.f32(s.this + 0x1c), (2.0f64 * 0.9) as f32);
        assert_eq!(calls_to(&s.log, ACTIVATE), [[s.body]]);
        assert_eq!(calls_to(&s.log, BRACKET).len(), 4);
        // The list for the target is looked up in the map.
        assert_eq!(calls_to(&s.log, MAP_GET_AT)[0][..2], [TARGET_MAP, target]);
    }

    #[test]
    fn update_finds_a_stick_point_and_pushes_neighbours_off_it() {
        let mut s = scene([5.0, 6.0, 7.0]);
        // One contact with another body on layer 1; its last point is at (3,0,4) with a
        // normal (1,0,0.25) whose z is under one half.
        let other = s.e.mem.alloc(0x40);
        s.e.mem.set_u32(other + 0x2c, 1);
        let owner = s.e.mem.alloc(0x40);
        let owner_table = s.e.mem.alloc(0x40);
        s.e.mem.set_u32(owner, owner_table);
        s.e.mem.set_u32(owner_table + 0x20, 0x0900_0020);
        record(&mut s.e, 0x0900_0020, 0xb, &s.log);
        let item = s.e.mem.alloc(0x20);
        s.e.mem.set_u32(item + 0xc, owner);
        s.e.mem.set_u32(item + 0x14, s.body);
        s.e.mem.set_u32(item + 0x18, other);
        let points = s.e.mem.alloc(0x80);
        s.e.mem.set_u16(points + 4, 1);
        for (lane, value) in [3.0f32, 0.0, 4.0].into_iter().enumerate() {
            s.e.mem.set_f32(points + 0x30 + 4 * lane as u32, value);
        }
        for (lane, value) in [1.0f32, 0.0, 0.25].into_iter().enumerate() {
            s.e.mem.set_f32(points + 0x40 + 4 * lane as u32, value);
        }
        s.e.mem.set_u32(owner + 0x28, points);
        let bytes = s.e.mem.alloc(4);
        s.e.mem.set_u32(owner + 0xc, bytes);
        s.e.mem.set_u32(owner + 0x10, 1);
        record(&mut s.e, CONTACT_COUNT, 1, &s.log);
        record(&mut s.e, CONTACT_GET, item, &s.log);
        // One neighbour: a second action whose vector (0.5,0,0) is scaled to (1,0,0).
        let other_holder = holder_with_vector(&mut s.e, 0x0900_00e4, [0.5, 0.0, 0.0]);
        let neighbour = action_for(&mut s.e, other_holder);
        let node = s.e.mem.alloc(8);
        s.e.mem.set_u32(node, neighbour);
        let target = s.e.mem.alloc(0x100);
        for row in 0..3u32 {
            s.e.mem.set_f32(target + 0x68 + row * 12 + row * 4, 1.0);
        }
        s.e.mem.set_u32(s.this + 0x10, target);
        s.e.mem.set_f32(s.this + 0x14, 2.0);
        s.e.mem.set_f32(s.this + 0x18, 3.0);
        s.e.register_double(MAP_GET_AT, move |e, a| {
            e.mem.set_u32(a[2], node);
            ret(1)
        });
        s.e.call(0x00cb_ccb0, &args![s.this, 0.5f32]);
        // Stick point and normal taken from the contact.
        assert_eq!(
            lanes(&s.e.mem.bytes(s.this + 0x24, 16))[..3],
            [3.0, 0.0, 4.0]
        );
        assert_eq!(
            lanes(&s.e.mem.bytes(s.this + 0x30, 16))[..3],
            [1.0, 0.0, 0.25]
        );
        // Zeroed velocity plus gravity (a stick point exists): z = -(0.5 * 2) * 3.
        assert_eq!(lanes(&s.velocity.borrow()), [0.0, 0.0, -3.0, 0.0]);
        // Force: the part of (3,0,4) along the normal, plus the neighbour push minus its
        // normal part, doubled by the scale.
        let hk_object = s.e.mem.u32(s.this + 8);
        assert_eq!(
            lanes(&s.e.mem.bytes(hk_object + 0x20, 16)),
            [8.0, 0.0, 3.5, 0.0]
        );
        assert_eq!(calls_to(&s.log, ACTIVATE), [[s.body]]);
        assert_eq!(calls_to(&s.log, CONTACT_GET), [[s.body, 0]]);
    }

    #[test]
    fn update_takes_the_player_reference_as_target_when_none_is_set() {
        let mut s = scene([0.0; 3]);
        let property = s.e.mem.alloc(4);
        s.e.mem.set_u32(property, 1);
        record(&mut s.e, GET_PROPERTY, property, &s.log);
        let reference = s.e.mem.alloc(0x10);
        let node = s.e.mem.alloc(0x20);
        s.e.mem.set_u32(reference + 8, node);
        record(&mut s.e, FIND_BY_KEY, reference, &s.log);
        record(&mut s.e, MAP_SET_AT, 0, &s.log);
        record(&mut s.e, POOL_ALLOCATE, 0, &s.log);
        let target = s.e.mem.alloc(0x100);
        s.e.mem.set_u32(node + 0x18, target);
        s.e.call(0x00cb_ccb0, &args![s.this, 0.5f32]);
        assert_eq!(s.e.mem.u32(s.this + 0x10), target);
        assert_eq!(calls_to(&s.log, FIND_BY_KEY), [[1, FIND_KEY]]);
        assert_eq!(calls_to(&s.log, GET_PROPERTY)[0][2], PROPERTY_ID);
    }

    #[test]
    fn update_keeps_the_property_holder_in_the_aligned_exe_frame() {
        let mut s = scene([0.0; 3]);
        let property = s.e.mem.alloc(4);
        record(&mut s.e, GET_PROPERTY, property, &s.log);
        // The next 0xd0-byte local is the frame (0xc0 bytes from a 16-byte boundary).
        let raw = s.e.next_stack_local(0xd0);
        s.e.call(0x00cb_ccb0, &args![s.this, 0.5f32]);
        let frame = (raw + 0xf) & !0xf;
        assert_eq!(calls_to(&s.log, GET_PROPERTY)[0][0], frame + 0x30);
    }
}
