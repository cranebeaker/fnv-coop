//! `BSHavok/bhkmalleableconstraint.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::bhklimitedhingeconstraint::{
    allocate_sized, enter_scope, increment, push_viewer_string, stream_transfer, ALLOCATE,
    POOL_ALLOCATE, POOL_OBJECT, REMOVE_REFERENCE,
};

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00ca9b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00cd_f7a0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00ca9b10(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00cd_f7a0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00ca9920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkMalleableConstraint::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00ca9860`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x10 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bhk_malleable_constraint_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00ca_9860;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x10u32]);
    }
    this
}

// Translated from 00ca9910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkMalleableConstraint::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_8310.
pub fn bhk_malleable_constraint_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_8310
}

/// `bhkMalleableConstraint` (0x10 bytes, as every `bhkConstraint`: the Havok object at `+8`,
/// `pInfo` at `+0xC`) owns a `hkMalleableConstraintCinfo` (RTTI-confirmed on vtable
/// `010c8288`, 0x14 bytes) whose `pData` (`+4`) is the 0x20-byte `ahkpMalleableConstraintData`
/// (vtable `010c82ec`, built on `hkpMalleableConstraintData` `010c82a4`): its wrapped
/// constraint data is at `+0xC`, the strength float at `+0x1C` and a
/// `hkpBridgeConstraintAtom` at `+0x10`. `pInfo` points at the cinfo's `pData` field
/// (cinfo `+4`), so the cinfo itself is `pInfo - 4`.
const VTABLE_CONSTRAINT: u32 = 0x0108_febc;
const VTABLE_MALLEABLE: u32 = 0x010c_81ac;
const VTABLE_CINFO: u32 = 0x010c_8288;
const VTABLE_DATA: u32 = 0x010c_82a4;
const VTABLE_DATA_ADAPTER: u32 = 0x010c_82ec;
const COUNT_CONSTRAINT: u32 = 0x0126_8188;
const COUNT_MALLEABLE: u32 = 0x0126_830c;
/// `bhkSerializable::Kill` (`00c876a0`), `CloneCritLock` (`00c876f0`), `CloneCritUnlock`
/// (`00c87700`), the `bhkRefObject` constructor (`00c87c30`) and `bhkConstraint`'s
/// destructor (`00c89640`), `CopyMembers` (`00c89730`, `(clone, process_info)`) and
/// `CopyCinfo` (`00c89c80`, one word).
const SERIALIZABLE_KILL: u32 = 0x00c8_76a0;
const CLONE_CRIT_LOCK: u32 = 0x00c8_76f0;
const CLONE_CRIT_UNLOCK: u32 = 0x00c8_7700;
const REF_OBJECT_CONSTRUCT: u32 = 0x00c8_7c30;
const CONSTRAINT_DESTRUCT: u32 = 0x00c8_9640;
const CONSTRAINT_COPY_MEMBERS: u32 = 0x00c8_9730;
const CONSTRAINT_COPY_CINFO: u32 = 0x00c8_9c80;
/// `hkConstraintCinfo`'s `CloneData` (`00c8a1b0`, `(target, info)`), `LoadBinary`
/// (`00c8a230`), `SaveBinary` (`00c8a280`) and `GetViewerStrings` (`00c8a560`), each with
/// the cinfo as `this` and one stream or strings word (`CloneData` two).
const CINFO_CLONE_DATA: u32 = 0x00c8_a1b0;
const CINFO_LOAD_BINARY: u32 = 0x00c8_a230;
const CINFO_SAVE_BINARY: u32 = 0x00c8_a280;
const CINFO_GET_VIEWER_STRINGS: u32 = 0x00c8_a560;
/// Cdecl `bhkConstraintManager` functions: `Clone` (`00cf4a30`, `(data, info)`),
/// `LoadBinary` (`00cf4af0`, `(stream)`, answers the data), the saving one (`00cf4bc0`,
/// `(stream, data)`) and `GetInfo` (`00cf4840`, `(type)`, answers a cinfo object).
const MANAGER_CLONE: u32 = 0x00cf_4a30;
const MANAGER_LOAD_BINARY: u32 = 0x00cf_4af0;
const MANAGER_SAVE_BINARY: u32 = 0x00cf_4bc0;
const MANAGER_GET_INFO: u32 = 0x00cf_4840;
/// `hkConstraintCinfo::SetData` (`00c89ac0`, `this` = the cinfo, one word).
const CINFO_SET_DATA: u32 = 0x00c8_9ac0;
/// `hkpBridgeConstraintAtom::init` (`00d21ce0`, `this` = the atom at `+0x10`, one word).
const BRIDGE_ATOM_INIT: u32 = 0x00d2_1ce0;
/// Takes a reference (`00c90510`, `ECX` = the object; Havok `hkreferencedobject.cpp` unit).
const ADD_REFERENCE: u32 = 0x00c9_0510;
/// `ApplyScale` of the ball-and-socket (`00cc96b0`) and hinge (`00cbd9e0`) cinfos, both
/// cdecl `(scale, data)`.
const BALL_AND_SOCKET_APPLY_SCALE: u32 = 0x00cc_96b0;
const HINGE_APPLY_SCALE: u32 = 0x00cb_d9e0;
/// `NiGetViewerString` for a float (`00a7d550`, cdecl `(name, bits)`) and the name text
/// "Strength" (`010678ac`).
const VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
const NAME_STRENGTH: u32 = 0x0106_78ac;
/// Constructor and `SetChild` of the data (`00ca9aa0`, `00ca9b40`).
const DATA_CONSTRUCT: u32 = 0x00ca_9aa0;
const DATA_SET_CHILD: u32 = 0x00ca_9b40;
/// `float` globals: the value `00ca98e0` answers and the default strength (`0.01`).
const MAX_LINEAR_IMPULSE: u32 = 0x0104_da48;
const DEFAULT_STRENGTH: u32 = 0x0101_3ea4;
/// The data allocation and the cinfo allocation (pool object at `011f6238`).
const DATA_SIZE: u32 = 0x20;
const CINFO_SIZE: u32 = 0x14;

// Translated from 00ca9860 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca9860 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkMalleableConstraint::~bhkMalleableConstraint` (Xbox PDB): sets the class vtable,
/// runs `bhkSerializable::Kill` (`00c876a0`), takes one off the instance counter
/// `0126830c` and runs the `bhkConstraint` destructor (`00c89640`).
pub fn bhk_malleable_constraint_destruct(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_MALLEABLE);
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.mem.u32(COUNT_MALLEABLE);
    e.mem.set_u32(COUNT_MALLEABLE, count.wrapping_sub(1));
    e.call(CONSTRAINT_DESTRUCT, &args![this]);
}

// Translated from 00ca98e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintData::getMaxLinearImpulse` (Xbox PDB, a shared body; slot `0x10` of the
/// `010c82ec` vtable): answers the `float` at `0104da48` in `ST0`.
pub fn fn_00ca98e0(e: &mut Engine) -> f32 {
    f32::from_bits(x87_quiet_bits(e.mem.u32(MAX_LINEAR_IMPULSE)))
}

// Translated from 00ca98f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintData::getNotifiedBodyIndex` (Xbox PDB, a shared body; slot `0x18` of the `010c82ec` vtable):
/// `OR AL,0xFF` answers 0xFF in `AL` (the rest of `EAX` is not set by the exe).
pub fn fn_00ca98f0(_e: &mut Engine) -> u8 {
    0xff
}

// Translated from 00ca9940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMalleableConstraintCinfo::ApplyScale` (Xbox PDB), cdecl `(scale, info)`: when `info`
/// and its wrapped data (`info + 0xC`) exist and `scale` is not 1.0 (a NaN is not), asks
/// the wrapped data for its type (slot `0x20`) and applies the scale through the
/// ball-and-socket cinfo's `ApplyScale` (`00cc96b0`) for type 0, or the hinge cinfo's
/// (`00cbd9e0`) for 1, 2 and 7; other types change nothing.
pub fn hk_malleable_constraint_cinfo_apply_scale(e: &mut Engine, scale: f32, info: Ptr) {
    if info.addr() == 0 {
        return;
    }
    let wrapped = e.mem.u32(info.addr() + 0xc);
    if wrapped == 0 || scale == 1.0 {
        return;
    }
    let kind = e.vcall(wrapped, 0x20, &args![]).u32();
    let bits = x87_quiet_bits(scale.to_bits());
    let data = e.mem.u32(info.addr() + 0xc);
    match kind {
        0 => {
            e.call(BALL_AND_SOCKET_APPLY_SCALE, &args![bits, data]);
        }
        1 | 2 | 7 => {
            e.call(HINGE_APPLY_SCALE, &args![bits, data]);
        }
        _ => {}
    }
}

// Translated from 00ca99f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca99f0 - the FS:[0] exception-unwinding frame is not modeled.
/// No map name: allocates a 0x10-byte `bhkMalleableConstraint` (`00aa13e0`) and, when that
/// succeeded, constructs it: the `bhkRefObject` constructor (`00c87c30`), the
/// `bhkConstraint` vtable (count `01268188` up by one), `+0xC` cleared, then the class
/// vtable (count `0126830c` up by one). Returns the object, or 0.
pub fn fn_00ca99f0(e: &mut Engine) -> u32 {
    let object = e.call(ALLOCATE, &args![0x10u32]).u32();
    if object == 0 {
        return 0;
    }
    construct(e, object);
    object
}

/// Constructs a `bhkMalleableConstraint` in `object` as `00ca99f0` does.
fn construct(e: &mut Engine, object: u32) {
    e.call(REF_OBJECT_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, VTABLE_CONSTRAINT);
    e.mem.set_u32(object + 0xc, 0);
    increment(e, COUNT_CONSTRAINT);
    e.mem.set_u32(object, VTABLE_MALLEABLE);
    increment(e, COUNT_MALLEABLE);
}

// Translated from 00ca9a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: constructs a `hkMalleableConstraintCinfo` (vtable `010c8288`,
/// RTTI-confirmed) in `this`: `+4` (data) cleared, `+8` (priority) 1, `+0xC` and `+0x10`
/// cleared. Returns `this`.
pub fn fn_00ca9a80(e: &mut Engine, this: Ptr) -> Ptr {
    let me = this.addr();
    e.mem.set_u32(me + 4, 0);
    e.mem.set_u32(me + 8, 1);
    e.mem.set_u32(me + 0xc, 0);
    e.mem.set_u32(me + 0x10, 0);
    e.mem.set_u32(me, VTABLE_CINFO);
    this
}

// Translated from 00ca9aa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca9aa0 - the FS:[0] exception-unwinding frame is not modeled.
/// `ahkpMalleableConstraintData::ahkpMalleableConstraintData` (Xbox PDB name; the vtable it
/// stores is `hkpMalleableConstraintData`'s, `010c82a4`): sets the words at `+6` (16-bit,
/// 1), `+8` and `+0xC` (cleared) and the 16-bit word at `+0x10` (1), initialises the
/// bridge atom at `+0x10` (`00d21ce0` with `this`) and copies the `float` at `01013ea4`
/// to the strength at `+0x1C`. Returns `this`.
pub fn ahkp_malleable_constraint_data_construct(e: &mut Engine, this: Ptr) -> Ptr {
    let me = this.addr();
    e.mem.set_u16(me + 6, 1);
    e.mem.set_u32(me + 8, 0);
    e.mem.set_u32(me, VTABLE_DATA);
    e.mem.set_u16(me + 0x10, 1);
    e.mem.set_u32(me + 0xc, 0);
    e.call(BRIDGE_ATOM_INIT, &args![me + 0x10, me]);
    let strength = x87_quiet_bits(e.mem.u32(DEFAULT_STRENGTH));
    e.mem.set_u32(me + 0x1c, strength);
    this
}

// Translated from 00ca9b40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca9b40 - the FS:[0] exception-unwinding frame is not modeled.
/// `ahkpMalleableConstraintData::SetChild` (Xbox PDB): takes a reference on `child`
/// (`00c90510`) when it is not null, releases the old wrapped data at `+0xC` (`00c905b0`)
/// when there was one and stores `child` there. The allocation-scope word is 0x10
/// meanwhile.
pub fn ahkp_malleable_constraint_data_set_child(e: &mut Engine, this: Ptr, child: u32) {
    let (word, saved) = enter_scope(e);
    if child != 0 {
        e.call(ADD_REFERENCE, &args![child]);
    }
    let old = e.mem.u32(this.addr() + 0xc);
    if old != 0 {
        e.call(REMOVE_REFERENCE, &args![old]);
    }
    e.mem.set_u32(this.addr() + 0xc, child);
    e.mem.set_u32(word, saved);
}

// Translated from 00ca9bd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca9bd0 - the FS:[0] exception-unwinding frame is not modeled.
/// `hkMalleableConstraintCinfo::CloneData` (Xbox PDB), slot `4` of the cinfo vtable
/// `010c8288` (`this` is the cinfo, `+4` its data): clones the data
/// into `target`, which is allocated (0x20 bytes, size word at `+4`), constructed
/// (`00ca9aa0`) and given the adapter vtable `010c82ec` first when 0. The wrapped data
/// (`+0xC`), when there is one, is cloned with `info` (`00cf4a30`) and, when that gave
/// something, set as `target`'s child (`00ca9b40`) and released. The strength at `+0x1C`
/// is copied (`FLD`/`FSTP`), the bridge atom at `+0x10` re-initialised (`00d21ce0`) and
/// `hkConstraintCinfo::CloneData` (`00c8a1b0`) finishes. The allocation-scope word is
/// 0x10 meanwhile.
pub fn hk_malleable_constraint_cinfo_clone_data(e: &mut Engine, this: Ptr, target: u32, info: u32) {
    let (word, saved) = enter_scope(e);
    let source = e.mem.u32(this.addr() + 4);
    let mut target = target;
    if target == 0 {
        target = allocate_sized(e, DATA_SIZE);
        e.call(DATA_CONSTRUCT, &args![target]);
        e.mem.set_u32(target, VTABLE_DATA_ADAPTER);
    }
    let wrapped = e.mem.u32(source + 0xc);
    if wrapped != 0 {
        let clone = e.call(MANAGER_CLONE, &args![wrapped, info]).u32();
        if clone != 0 {
            e.call(DATA_SET_CHILD, &args![target, clone]);
            e.call(REMOVE_REFERENCE, &args![clone]);
        }
    }
    let strength = x87_quiet_bits(e.mem.u32(source + 0x1c));
    e.mem.set_u32(target + 0x1c, strength);
    e.call(BRIDGE_ATOM_INIT, &args![target + 0x10, target]);
    e.call(CINFO_CLONE_DATA, &args![this, target, info]);
    e.mem.set_u32(word, saved);
}

// Translated from 00ca9ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMalleableConstraintCinfo::LoadBinary` (Xbox PDB), slot `8` of the cinfo vtable: runs
/// `hkConstraintCinfo::LoadBinary` (`00c8a230`), loads the wrapped constraint data from the
/// stream (`00cf4af0`), sets it as the data's child (`00ca9b40`), re-initialises the bridge
/// atom (`00d21ce0`), releases the loaded data (`00c905b0`) and reads the strength (4
/// bytes, counted) through the reader at `stream + 0x24c`.
pub fn hk_malleable_constraint_cinfo_load_binary(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(CINFO_LOAD_BINARY, &args![this, stream]);
    let data = e.mem.u32(this.addr() + 4);
    let loaded = e.call(MANAGER_LOAD_BINARY, &args![stream]).u32();
    e.call(DATA_SET_CHILD, &args![data, loaded]);
    e.call(BRIDGE_ATOM_INIT, &args![data + 0x10, data]);
    e.call(REMOVE_REFERENCE, &args![loaded]);
    let io = e.mem.u32(stream + 0x24c);
    stream_transfer(e, io, 8, data + 0x1c, 4, true);
}

// Translated from 00ca9d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMalleableConstraintCinfo::SaveBinary` (Xbox PDB), slot `0xC` of the cinfo vtable: runs
/// `hkConstraintCinfo::SaveBinary` (`00c8a280`), saves the wrapped constraint data to the
/// stream (`00cf4bc0`, `(stream, data)`) and writes the strength (4 bytes, counted)
/// through the writer at `stream + 0x250`.
pub fn hk_malleable_constraint_cinfo_save_binary(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(CINFO_SAVE_BINARY, &args![this, stream]);
    let data = e.mem.u32(this.addr() + 4);
    let wrapped = e.mem.u32(data + 0xc);
    e.call(MANAGER_SAVE_BINARY, &args![stream, wrapped]);
    let io = e.mem.u32(stream + 0x250);
    stream_transfer(e, io, 0xc, data + 0x1c, 4, true);
}

// Translated from 00ca9da0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca9da0 - the FS:[0] exception-unwinding frame is not modeled.
/// `hkMalleableConstraintCinfo::CreateData` (Xbox PDB), slot `0x10` of the cinfo vtable: when the data (`+4`)
/// is empty, allocates 0x20 bytes (size word at `+4`), constructs the data there
/// (`00ca9aa0`), gives it the adapter vtable `010c82ec` and keeps it. Answers 0x20. The
/// allocation-scope word is 0x10 meanwhile.
pub fn hk_malleable_constraint_cinfo_create_data(e: &mut Engine, this: Ptr) -> u32 {
    let (word, saved) = enter_scope(e);
    if e.mem.u32(this.addr() + 4) == 0 {
        let block = allocate_sized(e, DATA_SIZE);
        e.call(DATA_CONSTRUCT, &args![block]);
        e.mem.set_u32(block, VTABLE_DATA_ADAPTER);
        e.mem.set_u32(this.addr() + 4, block);
    }
    e.mem.set_u32(word, saved);
    DATA_SIZE
}

// Translated from 00ca9e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMalleableConstraintCinfo::SetWrappedConstraintData` (Xbox PDB): when the cinfo has
/// data (`+4`), a tail jump to `SetChild` (`00ca9b40`) on it with the one stack word;
/// otherwise nothing.
pub fn hk_malleable_constraint_cinfo_set_wrapped_constraint_data(
    e: &mut Engine,
    this: Ptr,
    child: u32,
) {
    let data = e.mem.u32(this.addr() + 4);
    if data != 0 {
        e.call(DATA_SET_CHILD, &args![data, child]);
    }
}

// Translated from 00ca9e60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca9e60 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkMalleableConstraint::CreateClone` (Xbox PDB), vtable slot `0x48`: inside the clone
/// lock (`00c876f0` / `00c87700`) allocates and constructs a new `bhkMalleableConstraint`
/// as `00ca99f0` does and copies the members into it (`00c89730`, `(clone,
/// process_info)`). Answers the clone (0 when the allocation failed). The
/// allocation-scope word is 0x10 meanwhile.
pub fn bhk_malleable_constraint_create_clone(e: &mut Engine, this: Ptr, process_info: u32) -> u32 {
    let (word, saved) = enter_scope(e);
    e.call(CLONE_CRIT_LOCK, &args![]);
    let block = e.call(ALLOCATE, &args![0x10u32]).u32();
    let clone = if block == 0 {
        0
    } else {
        construct(e, block);
        block
    };
    e.call(CONSTRAINT_COPY_MEMBERS, &args![this, clone, process_info]);
    e.call(CLONE_CRIT_UNLOCK, &args![]);
    e.mem.set_u32(word, saved);
    clone
}

// Translated from 00ca9f40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca9f40 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkMalleableConstraint::CreateCinfo` (Xbox PDB), vtable slot `0xB4`: `created` (a
/// byte) is set to whether `pInfo` was empty. If so, a 0x14-byte cinfo is allocated from
/// the pool (`00aa3e40`) and initialised (vtable `010c8288`, `+4` data empty, `+8`
/// priority 1, `+0xC` and `+0x10` empty), `pInfo` takes its `+4` address (0 when the
/// allocation failed) and, when the Havok object at `+8` exists, `CopyCinfo` (`00c89c80`)
/// is given the cinfo (0 for a failed allocation). Answers `pInfo`. The
/// allocation-scope word is 0x10 meanwhile.
pub fn bhk_malleable_constraint_create_cinfo(e: &mut Engine, this: Ptr, created: u32) -> u32 {
    let (word, saved) = enter_scope(e);
    let me = this.addr();
    let was_empty = e.mem.u32(me + 0xc) == 0;
    e.mem.set_u8(created, was_empty as u8);
    if was_empty {
        let block = e.call(POOL_ALLOCATE, &args![POOL_OBJECT, CINFO_SIZE]).u32();
        let info = if block != 0 {
            e.mem.set_u32(block + 8, 1);
            e.mem.set_u32(block + 0xc, 0);
            e.mem.set_u32(block + 0x10, 0);
            e.mem.set_u32(block, VTABLE_CINFO);
            e.mem.set_u32(block + 4, 0);
            block + 4
        } else {
            0
        };
        e.mem.set_u32(me + 0xc, info);
        if e.mem.u32(me + 8) != 0 {
            let cinfo = if info != 0 { info - 4 } else { 0 };
            e.call(CONSTRAINT_COPY_CINFO, &args![this, cinfo]);
        }
    }
    e.mem.set_u32(word, saved);
    e.mem.u32(me + 0xc)
}

// Translated from 00caa010 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00caa010 - the FS:[0] exception-unwinding frame is not modeled.
/// `hkMalleableConstraintCinfo::GetViewerStrings` (Xbox PDB), slot `0x14` of the cinfo vtable: runs
/// `hkConstraintCinfo::GetViewerStrings` (`00c8a560`), adds the strength at the data's
/// `+0x1C` (0.0 without data) under the name "Strength" (`00a7d550`), then, when the
/// data has wrapped constraint data (`+0xC`), builds the cinfo for its type (slot `0x20`,
/// `00cf4840`), gives it the data (`00c89ac0`), has it add its own viewer strings (slot
/// `0x14`) and deletes it (slot 0 with 1). The allocation-scope word is 0x10 meanwhile.
pub fn hk_malleable_constraint_cinfo_get_viewer_strings(e: &mut Engine, this: Ptr, strings: u32) {
    let (word, saved) = enter_scope(e);
    e.call(CINFO_GET_VIEWER_STRINGS, &args![this, strings]);
    let data = e.mem.u32(this.addr() + 4);
    let strength = if data != 0 {
        x87_quiet_bits(e.mem.u32(data + 0x1c))
    } else {
        0
    };
    let text = e
        .call(
            VIEWER_STRING_FLOAT,
            &args![NAME_STRENGTH, x87_quiet_bits(strength)],
        )
        .u32();
    push_viewer_string(e, strings, text);
    let data = e.mem.u32(this.addr() + 4);
    let wrapped = if data != 0 { e.mem.u32(data + 0xc) } else { 0 };
    if wrapped != 0 {
        let kind = e.vcall(wrapped, 0x20, &args![]).u32();
        let cinfo = e.call(MANAGER_GET_INFO, &args![kind]).u32();
        if cinfo != 0 {
            e.call(CINFO_SET_DATA, &args![cinfo, wrapped]);
            e.vcall(cinfo, 0x14, &args![strings]);
            e.vcall(cinfo, 0, &args![1u32]);
        }
    }
    e.mem.set_u32(word, saved);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ca9910, bhk_malleable_constraint_get_r_t_t_i() -> u32),
        entry!(0x00ca_9b10, deleting_destructor_00ca9b10(Ptr, u8) -> Ptr),
        entry!(0x00ca9920, bhk_malleable_constraint_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00ca9860, bhk_malleable_constraint_destruct(Ptr)),
        entry!(0x00ca98e0, fn_00ca98e0() -> f32),
        entry!(0x00ca98f0, fn_00ca98f0() -> u8),
        entry!(
            0x00ca9940,
            hk_malleable_constraint_cinfo_apply_scale(f32, Ptr)
        ),
        entry!(0x00ca99f0, fn_00ca99f0() -> u32),
        entry!(0x00ca9a80, fn_00ca9a80(Ptr) -> Ptr),
        entry!(0x00ca9aa0, ahkp_malleable_constraint_data_construct(Ptr) -> Ptr),
        entry!(
            0x00ca9b40,
            ahkp_malleable_constraint_data_set_child(Ptr, u32)
        ),
        entry!(
            0x00ca9bd0,
            hk_malleable_constraint_cinfo_clone_data(Ptr, u32, u32)
        ),
        entry!(
            0x00ca9ce0,
            hk_malleable_constraint_cinfo_load_binary(Ptr, u32)
        ),
        entry!(
            0x00ca9d50,
            hk_malleable_constraint_cinfo_save_binary(Ptr, u32)
        ),
        entry!(0x00ca9da0, hk_malleable_constraint_cinfo_create_data(Ptr) -> u32),
        entry!(
            0x00ca9e50,
            hk_malleable_constraint_cinfo_set_wrapped_constraint_data(Ptr, u32)
        ),
        entry!(0x00ca9e60, bhk_malleable_constraint_create_clone(Ptr, u32) -> u32),
        entry!(0x00ca9f40, bhk_malleable_constraint_create_cinfo(Ptr, u32) -> u32),
        entry!(
            0x00caa010,
            hk_malleable_constraint_cinfo_get_viewer_strings(Ptr, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
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
    fn deleting_destructor_00ca9b10_frees_only_when_bit_zero_is_set() {
        check(0x00ca_9b10, 0x00cd_f7a0);
    }

    #[test]
    fn bhk_malleable_constraint_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00ca_9860, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00ca9920, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00ca9920, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x10]]);
        assert_eq!(calls_to(&e, 0x00ca_9860), vec![vec![this]; 2]);
    }

    #[test]
    fn bhk_malleable_constraint_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00ca_9910, &args![]).u32(), 0x0126_8310);
    }
}

#[cfg(test)]
mod cinfo_tests {
    use super::super::bhklimitedhingeconstraint::support::*;
    use super::*;
    use std::rc::Rc;

    const STRINGS_SET_AT: u32 = 0x0096_ae90;

    /// A cinfo whose data (`+4`) has the given wrapped constraint data and strength bits.
    fn cinfo_with_data(e: &mut Engine, wrapped: u32, strength: u32) -> (u32, u32) {
        let data = e.mem.alloc(DATA_SIZE);
        e.mem.set_u32(data + 0xc, wrapped);
        e.mem.set_u32(data + 0x1c, strength);
        let cinfo = e.mem.alloc(0x14);
        e.mem.set_u32(cinfo + 4, data);
        (cinfo, data)
    }

    #[test]
    fn destruct_sets_the_vtable_kills_and_counts_down() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        e.set_global(COUNT_MALLEABLE, 5u32);
        let log: Calls = Rc::default();
        record(&mut e, SERIALIZABLE_KILL, 0, &log);
        record(&mut e, CONSTRAINT_DESTRUCT, 0, &log);
        let this = e.mem.alloc(0x10);
        e.call(0x00ca_9860, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_MALLEABLE);
        assert_eq!(e.global::<u32>(COUNT_MALLEABLE), 4);
        let order: Vec<u32> = log.borrow().iter().map(|c| c.0).collect();
        assert_eq!(order, [SERIALIZABLE_KILL, CONSTRAINT_DESTRUCT]);
        assert_eq!(calls_to(&log, CONSTRAINT_DESTRUCT), [[this]]);
    }

    #[test]
    fn fn_00ca98e0_answers_the_float_global_quieted() {
        let mut e = Engine::new();
        e.map(0x0104_d000, 0x1000);
        e.set_global(MAX_LINEAR_IMPULSE, 1.5f32);
        assert_eq!(e.call(0x00ca_98e0, &args![]).f32(), 1.5);
        e.set_global(MAX_LINEAR_IMPULSE, 0x7f80_0001u32);
        let answer = e.call(0x00ca_98e0, &args![]).f32();
        assert!(answer.is_nan());
    }

    #[test]
    fn fn_00ca98f0_answers_ff_in_al() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00ca_98f0, &args![]).u32() & 0xff, 0xff);
    }

    #[test]
    fn apply_scale_picks_the_cinfo_by_the_data_type() {
        let mut e = Engine::new();
        let log: Calls = Rc::default();
        record(&mut e, BALL_AND_SOCKET_APPLY_SCALE, 0, &log);
        record(&mut e, HINGE_APPLY_SCALE, 0, &log);
        let kind = Rc::new(std::cell::Cell::new(0u32));
        let answer = Rc::clone(&kind);
        e.register_double(0x0900_0020, move |_, _| ret(answer.get()));
        let wrapped = vobject(&mut e, 0x20, &[(0x20, 0x0900_0020)]);
        let info = e.mem.alloc(0x10);
        e.mem.set_u32(info + 0xc, wrapped);
        let scale = 2.0f32.to_bits();
        e.call(0x00ca_9940, &args![2.0f32, info]);
        assert_eq!(
            calls_to(&log, BALL_AND_SOCKET_APPLY_SCALE),
            [[scale, wrapped]]
        );
        assert!(calls_to(&log, HINGE_APPLY_SCALE).is_empty());
        for (n, type_id) in [1u32, 2, 7].into_iter().enumerate() {
            kind.set(type_id);
            e.call(0x00ca_9940, &args![2.0f32, info]);
            assert_eq!(calls_to(&log, HINGE_APPLY_SCALE).len(), n + 1);
        }
        assert_eq!(calls_to(&log, HINGE_APPLY_SCALE)[0], [scale, wrapped]);
        // The other types, and 1.0, do nothing; a NaN is not 1.0.
        for type_id in [3u32, 4, 5, 6, 8, 0x100] {
            kind.set(type_id);
            e.call(0x00ca_9940, &args![2.0f32, info]);
        }
        kind.set(0);
        e.call(0x00ca_9940, &args![1.0f32, info]);
        assert_eq!(calls_to(&log, BALL_AND_SOCKET_APPLY_SCALE).len(), 1);
        e.call(0x00ca_9940, &args![f32::NAN, info]);
        assert_eq!(calls_to(&log, BALL_AND_SOCKET_APPLY_SCALE).len(), 2);
        assert_eq!(calls_to(&log, HINGE_APPLY_SCALE).len(), 3);
        // No info, or no wrapped data: nothing.
        e.call(0x00ca_9940, &args![2.0f32, 0u32]);
        e.mem.set_u32(info + 0xc, 0);
        e.call(0x00ca_9940, &args![2.0f32, info]);
        assert_eq!(calls_to(&log, BALL_AND_SOCKET_APPLY_SCALE).len(), 2);
    }

    #[test]
    fn fn_00ca99f0_builds_the_object_and_counts_it() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        let object = e.mem.alloc(0x10);
        let log: Calls = Rc::default();
        record(&mut e, ALLOCATE, object, &log);
        record(&mut e, REF_OBJECT_CONSTRUCT, 0, &log);
        e.mem.set_u32(object + 0xc, 9);
        assert_eq!(e.call(0x00ca_99f0, &args![]).u32(), object);
        assert_eq!(calls_to(&log, ALLOCATE), [[0x10]]);
        assert_eq!(calls_to(&log, REF_OBJECT_CONSTRUCT), [[object]]);
        assert_eq!(e.mem.u32(object), VTABLE_MALLEABLE);
        assert_eq!(e.mem.u32(object + 0xc), 0);
        assert_eq!(e.global::<u32>(COUNT_CONSTRAINT), 1);
        assert_eq!(e.global::<u32>(COUNT_MALLEABLE), 1);
        record(&mut e, ALLOCATE, 0, &log);
        assert_eq!(e.call(0x00ca_99f0, &args![]).u32(), 0);
        assert_eq!(calls_to(&log, REF_OBJECT_CONSTRUCT).len(), 1);
    }

    #[test]
    fn fn_00ca9a80_initialises_the_cinfo() {
        let mut e = Engine::new();
        let cinfo = e.mem.alloc(0x14);
        for word in 0..5 {
            e.mem.set_u32(cinfo + 4 * word, 0xdead);
        }
        assert_eq!(e.call(0x00ca_9a80, &args![cinfo]).u32(), cinfo);
        let words: Vec<u32> = (0..5).map(|i| e.mem.u32(cinfo + 4 * i)).collect();
        assert_eq!(words, [VTABLE_CINFO, 0, 1, 0, 0]);
    }

    #[test]
    fn data_construct_sets_the_words_and_the_default_strength() {
        let mut e = Engine::new();
        e.map(0x0101_3000, 0x1000);
        e.set_global(DEFAULT_STRENGTH, 0x7f80_0001u32);
        let log: Calls = Rc::default();
        record(&mut e, BRIDGE_ATOM_INIT, 0, &log);
        let data = e.mem.alloc(DATA_SIZE);
        for word in 0..8 {
            e.mem.set_u32(data + 4 * word, 0xffff_ffff);
        }
        assert_eq!(e.call(0x00ca_9aa0, &args![data]).u32(), data);
        assert_eq!(e.mem.u32(data), VTABLE_DATA);
        assert_eq!(e.mem.u16(data + 6), 1);
        assert_eq!(e.mem.u32(data + 8), 0);
        assert_eq!(e.mem.u32(data + 0xc), 0);
        assert_eq!(e.mem.u16(data + 0x10), 1);
        // A signalling NaN default comes out quiet.
        assert_eq!(e.mem.u32(data + 0x1c), 0x7fc0_0001);
        assert_eq!(calls_to(&log, BRIDGE_ATOM_INIT), [[data + 0x10, data]]);
    }

    #[test]
    fn set_child_swaps_the_reference() {
        let (mut e, word, _) = engine_with_allocator();
        let log: Calls = Rc::default();
        record(&mut e, ADD_REFERENCE, 0, &log);
        record(&mut e, REMOVE_REFERENCE, 0, &log);
        let data = e.mem.alloc(DATA_SIZE);
        e.call(0x00ca_9b40, &args![data, 0x5000u32]);
        assert_eq!(e.mem.u32(data + 0xc), 0x5000);
        assert_eq!(calls_to(&log, ADD_REFERENCE), [[0x5000]]);
        assert!(calls_to(&log, REMOVE_REFERENCE).is_empty());
        e.call(0x00ca_9b40, &args![data, 0x6000u32]);
        assert_eq!(e.mem.u32(data + 0xc), 0x6000);
        assert_eq!(calls_to(&log, REMOVE_REFERENCE), [[0x5000]]);
        // A null child only releases the old one.
        e.call(0x00ca_9b40, &args![data, 0u32]);
        assert_eq!(e.mem.u32(data + 0xc), 0);
        assert_eq!(calls_to(&log, ADD_REFERENCE).len(), 2);
        assert_eq!(calls_to(&log, REMOVE_REFERENCE), [[0x5000], [0x6000]]);
        assert_eq!(e.mem.u32(word), 0x77);
    }

    #[test]
    fn clone_data_clones_the_wrapped_data_and_copies_the_strength() {
        let (mut e, word, block) = engine_with_allocator();
        let log: Calls = Rc::default();
        record(&mut e, DATA_CONSTRUCT, 0, &log);
        record(&mut e, MANAGER_CLONE, 0x7000, &log);
        record(&mut e, DATA_SET_CHILD, 0, &log);
        record(&mut e, REMOVE_REFERENCE, 0, &log);
        record(&mut e, BRIDGE_ATOM_INIT, 0, &log);
        record(&mut e, CINFO_CLONE_DATA, 0, &log);
        let (cinfo, _) = cinfo_with_data(&mut e, 0x4000, 0x7f80_0001);
        // A missing target is allocated, constructed and given the adapter vtable.
        e.call(0x00ca_9bd0, &args![cinfo, 0u32, 0x99u32]);
        assert_eq!(e.mem.u16(block + 4), 0x20);
        assert_eq!(e.mem.u32(block), VTABLE_DATA_ADAPTER);
        assert_eq!(calls_to(&log, DATA_CONSTRUCT), [[block]]);
        assert_eq!(calls_to(&log, MANAGER_CLONE), [[0x4000, 0x99]]);
        assert_eq!(calls_to(&log, DATA_SET_CHILD), [[block, 0x7000]]);
        assert_eq!(calls_to(&log, REMOVE_REFERENCE), [[0x7000]]);
        assert_eq!(e.mem.u32(block + 0x1c), 0x7fc0_0001);
        assert_eq!(calls_to(&log, BRIDGE_ATOM_INIT), [[block + 0x10, block]]);
        assert_eq!(calls_to(&log, CINFO_CLONE_DATA), [[cinfo, block, 0x99]]);
        assert_eq!(e.mem.u32(word), 0x77);
        // An existing target is used as is; a failed clone sets no child.
        record(&mut e, MANAGER_CLONE, 0, &log);
        let target = e.mem.alloc(DATA_SIZE);
        e.call(0x00ca_9bd0, &args![cinfo, target, 0x98u32]);
        assert_eq!(calls_to(&log, DATA_CONSTRUCT).len(), 1);
        assert_eq!(calls_to(&log, DATA_SET_CHILD).len(), 1);
        assert_eq!(e.mem.u32(target + 0x1c), 0x7fc0_0001);
        // Nothing wrapped: no clone is asked for.
        let (plain, _) = cinfo_with_data(&mut e, 0, 0);
        e.call(0x00ca_9bd0, &args![plain, target, 0x97u32]);
        assert_eq!(calls_to(&log, MANAGER_CLONE).len(), 2);
    }

    #[test]
    fn load_binary_loads_the_wrapped_data_and_the_strength() {
        let mut e = Engine::new();
        let log: Calls = Rc::default();
        let stream = stream_fixture(&mut e, &log);
        let io = e.mem.u32(stream + 0x24c);
        record(&mut e, CINFO_LOAD_BINARY, 0, &log);
        record(&mut e, MANAGER_LOAD_BINARY, 0x7000, &log);
        record(&mut e, DATA_SET_CHILD, 0, &log);
        record(&mut e, BRIDGE_ATOM_INIT, 0, &log);
        record(&mut e, REMOVE_REFERENCE, 0, &log);
        let (cinfo, data) = cinfo_with_data(&mut e, 0, 0);
        e.call(0x00ca_9ce0, &args![cinfo, stream]);
        assert_eq!(calls_to(&log, CINFO_LOAD_BINARY), [[cinfo, stream]]);
        assert_eq!(calls_to(&log, MANAGER_LOAD_BINARY), [[stream]]);
        assert_eq!(calls_to(&log, DATA_SET_CHILD), [[data, 0x7000]]);
        assert_eq!(calls_to(&log, BRIDGE_ATOM_INIT), [[data + 0x10, data]]);
        assert_eq!(calls_to(&log, REMOVE_REFERENCE), [[0x7000]]);
        let reads = calls_to(&log, 0x0900_0008);
        assert_eq!(reads.len(), 1);
        assert_eq!(reads[0][..3], [io, data + 0x1c, 4]);
        assert_eq!(reads[0][4], 1);
        assert_eq!(e.mem.u32(io + 4), 4);
        assert_eq!(e.mem.u32(data + 0x1c), 0x1312_1110);
    }

    #[test]
    fn save_binary_saves_the_wrapped_data_and_the_strength() {
        let mut e = Engine::new();
        let log: Calls = Rc::default();
        let stream = stream_fixture(&mut e, &log);
        let io = e.mem.u32(stream + 0x250);
        record(&mut e, CINFO_SAVE_BINARY, 0, &log);
        record(&mut e, MANAGER_SAVE_BINARY, 0, &log);
        let (cinfo, data) = cinfo_with_data(&mut e, 0x4000, 0x3f80_0000);
        e.call(0x00ca_9d50, &args![cinfo, stream]);
        assert_eq!(calls_to(&log, CINFO_SAVE_BINARY), [[cinfo, stream]]);
        assert_eq!(calls_to(&log, MANAGER_SAVE_BINARY), [[stream, 0x4000]]);
        let writes = calls_to(&log, 0x0900_000c);
        assert_eq!(writes.len(), 1);
        assert_eq!(writes[0][..3], [io, data + 0x1c, 4]);
        assert_eq!(e.mem.u32(io + 4), 4);
    }

    #[test]
    fn create_data_builds_the_data_once() {
        let (mut e, word, block) = engine_with_allocator();
        let log: Calls = Rc::default();
        record(&mut e, DATA_CONSTRUCT, 0, &log);
        let cinfo = e.mem.alloc(0x14);
        assert_eq!(e.call(0x00ca_9da0, &args![cinfo]).u32(), 0x20);
        assert_eq!(e.mem.u32(cinfo + 4), block);
        assert_eq!(e.mem.u32(block), VTABLE_DATA_ADAPTER);
        assert_eq!(e.mem.u16(block + 4), 0x20);
        assert_eq!(calls_to(&log, DATA_CONSTRUCT), [[block]]);
        assert_eq!(e.call(0x00ca_9da0, &args![cinfo]).u32(), 0x20);
        assert_eq!(calls_to(&log, DATA_CONSTRUCT).len(), 1);
        assert_eq!(e.mem.u32(word), 0x77);
    }

    #[test]
    fn set_wrapped_constraint_data_forwards_only_with_data() {
        let mut e = Engine::new();
        let log: Calls = Rc::default();
        record(&mut e, DATA_SET_CHILD, 0, &log);
        let cinfo = e.mem.alloc(0x14);
        e.call(0x00ca_9e50, &args![cinfo, 0x1234u32]);
        assert!(calls_to(&log, DATA_SET_CHILD).is_empty());
        e.mem.set_u32(cinfo + 4, 0x8000);
        e.call(0x00ca_9e50, &args![cinfo, 0x1234u32]);
        assert_eq!(calls_to(&log, DATA_SET_CHILD), [[0x8000, 0x1234]]);
    }

    #[test]
    fn create_clone_copies_the_members_inside_the_lock() {
        let (mut e, word, _) = engine_with_allocator();
        e.map(0x0126_8000, 0x1000);
        let clone = e.mem.alloc(0x10);
        let log: Calls = Rc::default();
        record(&mut e, CLONE_CRIT_LOCK, 0, &log);
        record(&mut e, CLONE_CRIT_UNLOCK, 0, &log);
        record(&mut e, ALLOCATE, clone, &log);
        record(&mut e, REF_OBJECT_CONSTRUCT, 0, &log);
        record(&mut e, CONSTRAINT_COPY_MEMBERS, 0, &log);
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00ca_9e60, &args![this, 0x55u32]).u32(), clone);
        assert_eq!(e.mem.u32(clone), VTABLE_MALLEABLE);
        assert_eq!(e.global::<u32>(COUNT_MALLEABLE), 1);
        assert_eq!(
            calls_to(&log, CONSTRAINT_COPY_MEMBERS),
            [[this, clone, 0x55]]
        );
        let order: Vec<u32> = log.borrow().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            [
                CLONE_CRIT_LOCK,
                ALLOCATE,
                REF_OBJECT_CONSTRUCT,
                CONSTRAINT_COPY_MEMBERS,
                CLONE_CRIT_UNLOCK
            ]
        );
        assert_eq!(e.mem.u32(word), 0x77);
        record(&mut e, ALLOCATE, 0, &log);
        assert_eq!(e.call(0x00ca_9e60, &args![this, 0x56u32]).u32(), 0);
        assert_eq!(calls_to(&log, CONSTRAINT_COPY_MEMBERS)[1], [this, 0, 0x56]);
    }

    #[test]
    fn create_cinfo_builds_it_once_and_hands_it_to_copy_cinfo() {
        let (mut e, word, _) = engine_with_allocator();
        let log: Calls = Rc::default();
        let block = e.mem.alloc(0x14);
        for offset in 0..5 {
            e.mem.set_u32(block + 4 * offset, 0xdead);
        }
        record(&mut e, POOL_ALLOCATE, block, &log);
        record(&mut e, CONSTRAINT_COPY_CINFO, 0, &log);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, 0x5000);
        let created = e.mem.alloc(4);
        assert_eq!(e.call(0x00ca_9f40, &args![this, created]).u32(), block + 4);
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(calls_to(&log, POOL_ALLOCATE), [[POOL_OBJECT, 0x14]]);
        let words: Vec<u32> = (0..5).map(|i| e.mem.u32(block + 4 * i)).collect();
        assert_eq!(words, [VTABLE_CINFO, 0, 1, 0, 0]);
        assert_eq!(calls_to(&log, CONSTRAINT_COPY_CINFO), [[this, block]]);
        assert_eq!(e.call(0x00ca_9f40, &args![this, created]).u32(), block + 4);
        assert_eq!(e.mem.u8(created), 0);
        assert_eq!(calls_to(&log, POOL_ALLOCATE).len(), 1);
        // No Havok object: no CopyCinfo; a failed allocation leaves pInfo 0.
        e.mem.set_u32(this + 0xc, 0);
        e.mem.set_u32(this + 8, 0);
        record(&mut e, POOL_ALLOCATE, 0, &log);
        assert_eq!(e.call(0x00ca_9f40, &args![this, created]).u32(), 0);
        assert_eq!(calls_to(&log, CONSTRAINT_COPY_CINFO).len(), 1);
        // A failed allocation still hands CopyCinfo a null cinfo.
        e.mem.set_u32(this + 8, 0x5000);
        e.call(0x00ca_9f40, &args![this, created]);
        assert_eq!(calls_to(&log, CONSTRAINT_COPY_CINFO)[1], [this, 0]);
        assert_eq!(e.mem.u32(word), 0x77);
    }

    #[test]
    fn get_viewer_strings_adds_the_strength_and_the_wrapped_cinfos_strings() {
        let (mut e, word, _) = engine_with_allocator();
        let log: Calls = Rc::default();
        record(&mut e, CINFO_GET_VIEWER_STRINGS, 0, &log);
        record(&mut e, VIEWER_STRING_FLOAT, 0x3000, &log);
        record(&mut e, CINFO_SET_DATA, 0, &log);
        record(&mut e, 0x0900_0030, 0, &log);
        record(&mut e, 0x0900_0034, 0, &log);
        record(&mut e, 0x0900_0038, 6, &log);
        let helper = vobject(&mut e, 0x20, &[(0x14, 0x0900_0030), (0, 0x0900_0034)]);
        record(&mut e, MANAGER_GET_INFO, helper, &log);
        e.register_double(STRINGS_SET_AT, |e, a| {
            let text = e.mem.u32(a[2]);
            e.mem.set_u32(0x0900_1000 + 4 * a[1], text);
            let size = e.mem.u16(a[0] + 0xa);
            e.mem.set_u16(a[0] + 0xa, size + 1);
            Ret::default()
        });
        e.map(0x0900_1000, 0x100);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 8);
        let wrapped = vobject(&mut e, 0x20, &[(0x20, 0x0900_0038)]);
        let (cinfo, _) = cinfo_with_data(&mut e, wrapped, 0x7f80_0001);
        e.call(0x00ca_a010, &args![cinfo, strings]);
        assert_eq!(calls_to(&log, CINFO_GET_VIEWER_STRINGS), [[cinfo, strings]]);
        assert_eq!(
            calls_to(&log, VIEWER_STRING_FLOAT),
            [[NAME_STRENGTH, 0x7fc0_0001]]
        );
        assert_eq!(e.mem.u32(0x0900_1000), 0x3000);
        assert_eq!(calls_to(&log, MANAGER_GET_INFO), [[6]]);
        assert_eq!(calls_to(&log, CINFO_SET_DATA), [[helper, wrapped]]);
        assert_eq!(calls_to(&log, 0x0900_0030), [[helper, strings]]);
        assert_eq!(calls_to(&log, 0x0900_0034), [[helper, 1]]);
        assert_eq!(e.mem.u32(word), 0x77);
        // Without data the strength is 0.0 and nothing else is asked.
        let empty = e.mem.alloc(0x14);
        e.call(0x00ca_a010, &args![empty, strings]);
        assert_eq!(calls_to(&log, VIEWER_STRING_FLOAT)[1], [NAME_STRENGTH, 0]);
        assert_eq!(calls_to(&log, MANAGER_GET_INFO).len(), 1);
        // A type nobody has a cinfo for: no further calls.
        record(&mut e, MANAGER_GET_INFO, 0, &log);
        e.call(0x00ca_a010, &args![cinfo, strings]);
        assert_eq!(calls_to(&log, CINFO_SET_DATA).len(), 1);
        assert_eq!(e.mem.u32(word), 0x77);
    }
}
