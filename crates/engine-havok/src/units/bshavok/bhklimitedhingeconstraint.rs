//! `BSHavok/bhklimitedhingeconstraint.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkLimitedHingeConstraint` (0x10 bytes: `phkObject` at `+8`, `pInfo` at `+0xC`) owns a
//! `hkLimitedHingeConstraintCinfo` (0x14 bytes, RTTI-confirmed on vtable `010c5dc0`) whose
//! `pData` (`+4`) is the 0xE0-byte `hkpLimitedHingeConstraintData`. `pInfo` points at the
//! cinfo's `pData` field (cinfo `+4`), so the cinfo itself is `pInfo - 4`.
//!
//! The functions that allocate or tear down Havok objects raise the per-thread allocation
//! word at `TLS + 0x2b4` to 0x10 for their duration and put the old value back. The
//! exception-unwinding frames (`FS:[0]` registrations) of the functions marked
//! `Translation incomplete` are not modeled.

#[allow(unused_imports)]
use crate::prelude::*;

/// Per-thread word the Havok-allocating functions set to 0x10 (`TLS + 0x2b4`).
const SCOPE_WORD: u32 = 0x2b4;
/// `hkMemoryRouter` lookup (`00c85750`): its `+0x10` is the allocator whose slot `+4`
/// allocates `(size)` and slot `+8` frees `(pointer, size)`.
pub(crate) const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// Pool allocator (`00aa3e40`, thiscall on the pool object at `011f6238`, one word).
pub(crate) const POOL_ALLOCATE: u32 = 0x00aa_3e40;
pub(crate) const POOL_OBJECT: u32 = 0x011f_6238;
/// `00aa13e0`: cdecl allocation of one size word.
pub(crate) const ALLOCATE: u32 = 0x00aa_13e0;
/// `hkpLimitedHingeConstraintData` constructor (`00cb9090`, `ECX` = the block, returns it)
/// and `setMotor` (`00cb8ac0`, one word).
const DATA_CONSTRUCT: u32 = 0x00cb_9090;
const DATA_SET_MOTOR: u32 = 0x00cb_8ac0;
/// `hkHingeConstraintCinfo::ApplyScale` (`00cbd9e0`, cdecl `(scale, data)`).
const APPLY_SCALE: u32 = 0x00cb_d9e0;
/// `hkReferencedObject::removeReference` (`00c905b0`).
pub(crate) const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkConstraintCinfo::CloneData` (`00c8a1b0`, `(data, info)`), `LoadBinary` (`00c8a230`),
/// `LoadMotor` (`00c8a330`, returns the motor), `SaveBinary` (`00c8a280`), `SaveMotor`
/// (`00c8a4b0`, `(motor, stream)`) and `GetViewerStrings` (`00c8a560`).
const CINFO_CLONE_DATA: u32 = 0x00c8_a1b0;
const CINFO_LOAD_BINARY: u32 = 0x00c8_a230;
const CINFO_LOAD_MOTOR: u32 = 0x00c8_a330;
const CINFO_SAVE_BINARY: u32 = 0x00c8_a280;
const CINFO_SAVE_MOTOR: u32 = 0x00c8_a4b0;
const CINFO_GET_VIEWER_STRINGS: u32 = 0x00c8_a560;
/// `00c8cdb0`: cdecl, one word (a 4x4 block of the cinfo after its byte swap).
const FIX_ROTATION_BLOCK: u32 = 0x00c8_cdb0;
/// `bhkSerializable::Kill` (`00c876a0`), `CloneCritLock` (`00c876f0`) and
/// `CloneCritUnlock` (`00c87700`); the `bhkRefObject` constructor (`00c87c30`);
/// `bhkConstraint`'s destructor (`00c89640`), `CopyMembers` (`00c89730`, `(clone,
/// process_info)`), `CopyCinfo` (`00c89c80`, one word) and the `setEntityB` forwarding
/// target (`00c89cf0`, one word).
const SERIALIZABLE_KILL: u32 = 0x00c8_76a0;
const CLONE_CRIT_LOCK: u32 = 0x00c8_76f0;
const CLONE_CRIT_UNLOCK: u32 = 0x00c8_7700;
const REF_OBJECT_CONSTRUCT: u32 = 0x00c8_7c30;
const CONSTRAINT_DESTRUCT: u32 = 0x00c8_9640;
const CONSTRAINT_COPY_MEMBERS: u32 = 0x00c8_9730;
const CONSTRAINT_COPY_CINFO: u32 = 0x00c8_9c80;
const CONSTRAINT_SET_ENTITY_B: u32 = 0x00c8_9cf0;
/// `NiPoint3` conversion from a Havok vector (`004a3970`, cdecl `(destination, source)`),
/// `NiPoint3::GetViewerString` (`00a59af0`, `ECX` = the point, one word), and the
/// `NiGetViewerString` overloads for a float (`00a7d550`, cdecl `(name, bits)`) and a
/// string (`00a7d5e0`, cdecl `(name, text)`).
const POINT_FROM_VECTOR: u32 = 0x004a_3970;
const POINT_GET_VIEWER_STRING: u32 = 0x00a5_9af0;
const VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
const VIEWER_STRING_TEXT: u32 = 0x00a7_d5e0;
/// The viewer-string array's grow (`0096ad30`) and set-at (`0096ae90`) methods.
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// The vtables of `bhkConstraint` and `bhkLimitedHingeConstraint`, and of
/// `hkLimitedHingeConstraintCinfo`, with the instance counters of the first two.
const VTABLE_CONSTRAINT: u32 = 0x0108_febc;
const VTABLE_LIMITED_HINGE: u32 = 0x010c_5ce4;
const VTABLE_CINFO: u32 = 0x010c_5dc0;
const COUNT_CONSTRAINT: u32 = 0x0126_8188;
const COUNT_LIMITED_HINGE: u32 = 0x0126_8318;
/// Size of the data block and of the cinfo allocation.
const DATA_SIZE: u32 = 0xe0;
const CINFO_SIZE: u32 = 0x14;
/// The viewer-string names, a degrees factor (`double` at `0102f248`), the "Active" and
/// "Inactive" words and the `float` the motor-less default of `+0xBC` becomes (`01016410`).
const NAME_PIVOT_IN_A: u32 = 0x010c_839c;
const NAME_PIVOT_IN_B: u32 = 0x010c_8390;
const NAME_AXLE_IN_B: u32 = 0x010c_8388;
const NAME_PERP_AXIS_IN_A: u32 = 0x010c_8378;
const NAME_PERP_AXIS_IN_B: u32 = 0x010c_8368;
const NAME_MIN_ANGLE: u32 = 0x010c_835c;
const NAME_MAX_ANGLE: u32 = 0x010c_8350;
const NAME_MAX_FRICTION: u32 = 0x010c_8340;
const NAME_MOTOR: u32 = 0x010c_8338;
const NAME_ANGLE: u32 = 0x010c_832c;
const TEXT_ACTIVE: u32 = 0x0109_c938;
const TEXT_INACTIVE: u32 = 0x010c_66e4;
const DEGREES_FACTOR: u32 = 0x0102_f248;
const DEFAULT_MAX_FRICTION: u32 = 0x0101_6410;

/// Sets the allocation-scope word to 0x10 and returns its address and old value.
pub(crate) fn enter_scope(e: &mut Engine) -> (u32, u32) {
    let word = e.tls() + SCOPE_WORD;
    let saved = e.mem.u32(word);
    e.mem.set_u32(word, 0x10);
    (word, saved)
}

/// The allocator of the memory router (`router + 0x10`).
pub(crate) fn allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// Allocates `size` bytes through the router's allocator (slot `4`) and keeps `size` as a
/// 16-bit word at `+4` of the block, as the Havok allocation sites do.
pub(crate) fn allocate_sized(e: &mut Engine, size: u32) -> u32 {
    let heap = allocator(e);
    let block = e.vcall(heap, 4, &args![size]).u32();
    e.mem.set_u16(block + 4, size as u16);
    block
}

/// Appends the viewer string `text` to the array `strings` (size and capacity are
/// 16-bit fields at `+0xA` and `+8`; `+0xE` is the grow step): grows it when full,
/// then sets the element at the old size.
pub(crate) fn push_viewer_string(e: &mut Engine, strings: u32, text: u32) {
    let size = u32::from(e.mem.u16(strings + 0xa));
    if size >= u32::from(e.mem.u16(strings + 8)) {
        let step = u32::from(e.mem.u16(strings + 0xe));
        e.call(STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

/// One transfer through the stream's function pointer at `io + offset` (`+8` reads,
/// `+0xC` writes): `(io, buffer, size, count_slot, 1)` or `(io, buffer, size, 0, 0)`, then
/// the stream position at `io + 4` advances by the bytes done.
pub(crate) fn stream_transfer(
    e: &mut Engine,
    io: u32,
    offset: u32,
    buffer: u32,
    size: u32,
    counted: bool,
) {
    let function = e.mem.u32(io + offset);
    let done = if counted {
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), size);
            e.call(function, &args![io, buffer, size, slot, 1u32]).u32()
        })
    } else {
        e.call(function, &args![io, buffer, size, 0u32, 0u32]).u32()
    };
    let position = e.mem.u32(io + 4);
    e.mem.set_u32(io + 4, position.wrapping_add(done));
}

/// Copies one `float` as `FLD` / `FSTP` does (a signalling NaN comes out quiet).
fn copy_float(e: &mut Engine, from: u32, to: u32) {
    let bits = x87_quiet_bits(e.mem.u32(from));
    e.mem.set_u32(to, bits);
}

/// Adds one to the 32-bit counter at `counter`.
pub(crate) fn increment(e: &mut Engine, counter: u32) {
    let count = e.mem.u32(counter);
    e.mem.set_u32(counter, count.wrapping_add(1));
}

// Translated from 00caa140 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00caa140 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkLimitedHingeConstraint::~bhkLimitedHingeConstraint` (Xbox PDB): sets the class
/// vtable, runs `bhkSerializable::Kill` (`00c876a0`), takes one off the instance counter
/// `01268318` and runs the `bhkConstraint` destructor (`00c89640`).
pub fn bhk_limited_hinge_constraint_destruct(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_LIMITED_HINGE);
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.mem.u32(COUNT_LIMITED_HINGE);
    e.mem.set_u32(COUNT_LIMITED_HINGE, count.wrapping_sub(1));
    e.call(CONSTRAINT_DESTRUCT, &args![this]);
}

// Translated from 00caa1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot `0xC8` of the `bhkLimitedHingeConstraint` vtable (map name
/// `bhkHingeLimitsConstraint::SetEntityB`, a shared body): a tail jump to `00c89cf0` with
/// `this` and its one stack word, which does nothing when `this` or the Havok object at
/// `+8` is null.
pub fn fn_00caa1b0(e: &mut Engine, this: Ptr, entity: u32) {
    e.call(CONSTRAINT_SET_ENTITY_B, &args![this, entity]);
}

// Translated from 00caa1c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00caa1c0 - the FS:[0] exception-unwinding frame is not modeled.
/// No map name: allocates a 0x10-byte `bhkLimitedHingeConstraint` (`00aa13e0`) and, when
/// that succeeded, constructs it: the `bhkRefObject` constructor (`00c87c30`), the
/// `bhkConstraint` vtable (count `01268188` up by one), `+0xC` cleared, then the class
/// vtable (count `01268318` up by one). Returns the object, or 0.
pub fn fn_00caa1c0(e: &mut Engine) -> u32 {
    let object = e.call(ALLOCATE, &args![0x10u32]).u32();
    if object == 0 {
        return 0;
    }
    e.call(REF_OBJECT_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, VTABLE_CONSTRAINT);
    e.mem.set_u32(object + 0xc, 0);
    increment(e, COUNT_CONSTRAINT);
    e.mem.set_u32(object, VTABLE_LIMITED_HINGE);
    increment(e, COUNT_LIMITED_HINGE);
    object
}

// Translated from 00caa250 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00caa250 - the FS:[0] exception-unwinding frame is not modeled.
/// `hkLimitedHingeConstraintCinfo::CloneData` (Xbox PDB), slot `4` of the cinfo vtable
/// (`this` is the cinfo, `+4` its data): copies the data block into `target`, which is
/// allocated (0xE0 bytes, size word at `+4`) and constructed first when 0. The copy is
/// the eight 16-byte blocks `+0x20..+0xA0` and the floats at `+0xBC`, `+0xC4` and `+0xC8`;
/// the scale `info + 0x10` is applied (`00cbd9e0`) unless it equals 1.0 (a NaN is not
/// equal). A motor at `+0xB0` is cloned through its slot `0xC` into `target`'s `setMotor`
/// and released, and `+0xA2` is copied. `hkConstraintCinfo::CloneData` (`00c8a1b0`)
/// finishes. The allocation-scope word is 0x10 meanwhile.
pub fn fn_00caa250(e: &mut Engine, this: Ptr, target: u32, info: u32) {
    let (word, saved) = enter_scope(e);
    let data = e.mem.u32(this.addr() + 4);
    let mut target = target;
    if target == 0 {
        let block = allocate_sized(e, DATA_SIZE);
        target = e.call(DATA_CONSTRUCT, &args![block]).u32();
    }
    for block in 2..10u32 {
        let bytes = e.mem.bytes(data + block * 0x10, 0x10);
        e.mem.write(target + block * 0x10, &bytes);
    }
    copy_float(e, data + 0xbc, target + 0xbc);
    copy_float(e, data + 0xc4, target + 0xc4);
    copy_float(e, data + 0xc8, target + 0xc8);
    let scale = x87_quiet_bits(e.mem.u32(info + 0x10));
    if f32::from_bits(scale) != 1.0 {
        e.call(APPLY_SCALE, &args![scale, target]);
    }
    let motor = e.mem.u32(data + 0xb0);
    if motor != 0 {
        let clone = e.vcall(motor, 0xc, &args![]).u32();
        e.call(DATA_SET_MOTOR, &args![target, clone]);
        e.call(REMOVE_REFERENCE, &args![clone]);
        let active = e.mem.u8(data + 0xa2);
        e.mem.set_u8(target + 0xa2, active);
    }
    e.call(CINFO_CLONE_DATA, &args![this, target, info]);
    e.mem.set_u32(word, saved);
}

// Translated from 00caa3f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00caa3f0 - the FS:[0] exception-unwinding frame is not modeled.
/// `hkLimitedHingeConstraintCinfo::CreateData` (Xbox PDB), slot `0x10` of the cinfo vtable
/// (`this` is the cinfo): when `+4` (the data) is empty, allocates 0xE0 bytes (size word
/// at `+4`), constructs the data block there (`00cb9090`) and keeps what that returns.
/// Answers 0xE0. The allocation-scope word is 0x10 meanwhile.
pub fn fn_00caa3f0(e: &mut Engine, this: Ptr) -> u32 {
    let (word, saved) = enter_scope(e);
    if e.mem.u32(this.addr() + 4) == 0 {
        let block = allocate_sized(e, DATA_SIZE);
        let data = e.call(DATA_CONSTRUCT, &args![block]).u32();
        e.mem.set_u32(this.addr() + 4, data);
    }
    e.mem.set_u32(word, saved);
    DATA_SIZE
}

// Translated from 00caa4a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00caa4a0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkLimitedHingeConstraint::CreateClone` (Xbox PDB), vtable slot `0x48`: inside the
/// clone lock (`00c876f0` / `00c87700`) allocates and constructs a new
/// `bhkLimitedHingeConstraint` as `00caa1c0` does, copies the members (`00c89730`,
/// `(clone, process_info)`) and, when `this` has a data block (the Havok object's `+0xC`
/// when it has one, else the first word of `pInfo`) whose `+0xB0` is not null, builds
/// the cinfo through slot `0xB4` and calls its slot `4` with `(0, process_info)`. Answers
/// the clone. The allocation-scope word is 0x10 meanwhile.
pub fn bhk_limited_hinge_constraint_create_clone(
    e: &mut Engine,
    this: Ptr,
    process_info: u32,
) -> u32 {
    let (word, saved) = enter_scope(e);
    e.call(CLONE_CRIT_LOCK, &args![]);
    let clone = e.call(ALLOCATE, &args![0x10u32]).u32();
    let clone = if clone == 0 {
        0
    } else {
        e.call(REF_OBJECT_CONSTRUCT, &args![clone]);
        e.mem.set_u32(clone, VTABLE_CONSTRAINT);
        e.mem.set_u32(clone + 0xc, 0);
        increment(e, COUNT_CONSTRAINT);
        e.mem.set_u32(clone, VTABLE_LIMITED_HINGE);
        increment(e, COUNT_LIMITED_HINGE);
        clone
    };
    e.call(CONSTRAINT_COPY_MEMBERS, &args![this, clone, process_info]);
    let me = this.addr();
    let hk_object = if me != 0 { e.mem.u32(me + 8) } else { 0 };
    let data = if hk_object != 0 {
        Some(e.mem.u32(hk_object + 0xc))
    } else {
        let info = e.mem.u32(me + 0xc);
        if info != 0 {
            Some(e.mem.u32(info))
        } else {
            None
        }
    };
    if let Some(data) = data {
        if data != 0 && e.mem.u32(data + 0xb0) != 0 {
            let info = e.with_stack(4, |e, created| e.vcall(me, 0xb4, &args![created]).u32());
            let cinfo = if info != 0 { info.wrapping_sub(4) } else { 0 };
            e.vcall(cinfo, 4, &args![0u32, process_info]);
        }
    }
    e.call(CLONE_CRIT_UNLOCK, &args![]);
    e.mem.set_u32(word, saved);
    clone
}

// Translated from 00caa5d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00caa5d0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkLimitedHingeConstraint::CreateCinfo` (Xbox PDB), vtable slot `0xB4`: `created`
/// (a byte) is set to whether `pInfo` was empty. If so, a 0x14-byte cinfo is allocated
/// from the pool (`00aa3e40`) and initialised (vtable `010c5dc0`, `+4` data empty, `+8`
/// priority 1, `+0xC` and `+0x10` empty), `pInfo` takes its `+4` address (0 when the
/// allocation failed) and, when the Havok object at `+8` exists, `CopyCinfo` (`00c89c80`)
/// is given the cinfo. Answers `pInfo`. The allocation-scope word is 0x10 meanwhile.
pub fn bhk_limited_hinge_constraint_create_cinfo(e: &mut Engine, this: Ptr, created: Ptr) -> u32 {
    let me = this.addr();
    let (word, saved) = enter_scope(e);
    let empty = e.mem.u32(me + 0xc) == 0;
    e.mem.set_u8(created.addr(), u8::from(empty));
    if empty {
        let block = e.call(POOL_ALLOCATE, &args![POOL_OBJECT, CINFO_SIZE]).u32();
        let mut info = 0;
        if block != 0 {
            e.mem.set_u32(block + 8, 1);
            e.mem.set_u32(block + 0xc, 0);
            e.mem.set_u32(block + 0x10, 0);
            e.mem.set_u32(block, VTABLE_CINFO);
            e.mem.set_u32(block + 4, 0);
            info = block + 4;
        }
        e.mem.set_u32(me + 0xc, info);
        if e.mem.u32(me + 8) != 0 {
            let cinfo = if info != 0 { info - 4 } else { 0 };
            e.call(CONSTRAINT_COPY_CINFO, &args![this, cinfo]);
        }
    }
    let info = e.mem.u32(me + 0xc);
    e.mem.set_u32(word, saved);
    info
}

// Translated from 00caa6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLimitedHingeConstraintCinfo::LoadBinary` (Xbox PDB), slot `8` of the cinfo vtable
/// (`this` is the cinfo, `stream` the `NiStream`, whose reader is at `+0x24C`): the base
/// `LoadBinary` (`00c8a230`), then reads 0x40 bytes to data `+0x20` and `+0x60`, and four
/// bytes each to `+0xC4`, `+0xC8` and `+0xBC`, advancing the reader's position. A stream
/// not in PC byte order (`stream + 0x1E5` not 1) swaps the words at `+0x50..+0x5C` and
/// `+0x90..+0x9C` and fixes the blocks at `+0x20` and `+0x60` (`00c8cdb0`). A stream
/// version above 0x1D loads a motor (`00c8a330`), gives it to `setMotor` and, when there
/// is one, reads the byte at `+0xA2` and releases it. A `+0xBC` of exactly 0 becomes the
/// `float` at `01016410`.
pub fn fn_00caa6a0(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(CINFO_LOAD_BINARY, &args![this, stream]);
    let data = e.mem.u32(this.addr() + 4);
    let io = e.mem.u32(stream + 0x24c);
    stream_transfer(e, io, 8, data + 0x20, 0x40, false);
    let io = e.mem.u32(stream + 0x24c);
    stream_transfer(e, io, 8, data + 0x60, 0x40, false);
    for offset in [0xc4, 0xc8, 0xbc] {
        let io = e.mem.u32(stream + 0x24c);
        stream_transfer(e, io, 8, data + offset, 4, true);
    }
    if e.mem.u8(stream + 0x1e5) != 1 {
        for (block, first) in [(data + 0x20, data + 0x50), (data + 0x60, data + 0x90)] {
            for word in 0..4 {
                let value = e.mem.u32(first + 4 * word);
                e.mem.set_u32(first + 4 * word, value.swap_bytes());
            }
            e.call(FIX_ROTATION_BLOCK, &args![block]);
        }
    }
    if e.mem.u32(stream + 4) > 0x1d {
        let motor = e.call(CINFO_LOAD_MOTOR, &args![this, stream]).u32();
        e.call(DATA_SET_MOTOR, &args![data, motor]);
        if motor != 0 {
            let io = e.mem.u32(stream + 0x24c);
            stream_transfer(e, io, 8, data + 0xa2, 1, true);
            e.call(REMOVE_REFERENCE, &args![motor]);
        }
    }
    if e.mem.f32(data + 0xbc) == 0.0 {
        copy_float(e, DEFAULT_MAX_FRICTION, data + 0xbc);
    }
}

// Translated from 00caa840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLimitedHingeConstraintCinfo::SaveBinary` (Xbox PDB), slot `0xC` of the cinfo vtable
/// (`this` is the cinfo, `stream` the `NiStream`, whose writer is at `+0x250`): the base
/// `SaveBinary` (`00c8a280`), then writes 0x40 bytes from data `+0x20` and `+0x60` and
/// four bytes each from `+0xC4`, `+0xC8` and `+0xBC`, the motor (`00c8a4b0`, `(motor,
/// stream)`) and, when there is a motor, the byte at `+0xA2`.
pub fn fn_00caa840(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(CINFO_SAVE_BINARY, &args![this, stream]);
    let data = e.mem.u32(this.addr() + 4);
    for (offset, size) in [(0x20, 0x40), (0x60, 0x40), (0xc4, 4), (0xc8, 4), (0xbc, 4)] {
        let io = e.mem.u32(stream + 0x250);
        stream_transfer(e, io, 0xc, data + offset, size, true);
    }
    let motor = e.mem.u32(data + 0xb0);
    e.call(CINFO_SAVE_MOTOR, &args![motor, stream]);
    if e.mem.u32(data + 0xb0) != 0 {
        let io = e.mem.u32(stream + 0x250);
        stream_transfer(e, io, 0xc, data + 0xa2, 1, true);
    }
}

/// `value` (a `float` read from the data) scaled to degrees the way the code does: the
/// x87 product with the `double` at `0102f248`, stored as a `float`, as bits.
fn degrees_bits(e: &mut Engine, value: u32) -> u32 {
    let factor = e.global::<f64>(DEGREES_FACTOR);
    let product = (f64::from(f32::from_bits(value)) * factor) as f32;
    x87_quiet_bits(product.to_bits())
}

// Translated from 00caa960 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00caa960 - the FS:[0] exception-unwinding frame is not modeled.
/// `hkLimitedHingeConstraintCinfo::GetViewerStrings` (Xbox PDB), slot `0x14` of the cinfo
/// vtable (`this` is the cinfo): the base strings (`00c8a560`), then, when there is a data
/// block, the points at `+0x50` (PivotInA), `+0x90` (PivotInB), `+0x60` (AxleInB), `+0x30`
/// and `+0x40` as `NiPoint3` viewer strings, MinAngle (`+0xC4`) and MaxAngle (`+0xC8`) in
/// degrees, MaxFriction (`+0xBC`), Motor as Active or Inactive (`+0xA2`) and the angle at
/// `+0xAC` in degrees. The allocation-scope word is 0x10 meanwhile.
pub fn fn_00caa960(e: &mut Engine, this: Ptr, strings: u32) {
    let (word, saved) = enter_scope(e);
    e.call(CINFO_GET_VIEWER_STRINGS, &args![this, strings]);
    let data = e.mem.u32(this.addr() + 4);
    if data != 0 {
        for (offset, name) in [
            (0x50, NAME_PIVOT_IN_A),
            (0x90, NAME_PIVOT_IN_B),
            (0x60, NAME_AXLE_IN_B),
            (0x30, NAME_PERP_AXIS_IN_A),
            (0x40, NAME_PERP_AXIS_IN_B),
        ] {
            let text = e.with_stack(12, |e, point| {
                e.call(POINT_FROM_VECTOR, &args![point, data + offset]);
                e.call(POINT_GET_VIEWER_STRING, &args![point, name]).u32()
            });
            push_viewer_string(e, strings, text);
        }
        let bits = e.mem.u32(data + 0xc4);
        let value = degrees_bits(e, bits);
        let text = e
            .call(VIEWER_STRING_FLOAT, &args![NAME_MIN_ANGLE, value])
            .u32();
        push_viewer_string(e, strings, text);
        let bits = e.mem.u32(data + 0xc8);
        let value = degrees_bits(e, bits);
        let text = e
            .call(VIEWER_STRING_FLOAT, &args![NAME_MAX_ANGLE, value])
            .u32();
        push_viewer_string(e, strings, text);
        let value = x87_quiet_bits(e.mem.u32(data + 0xbc));
        let text = e
            .call(VIEWER_STRING_FLOAT, &args![NAME_MAX_FRICTION, value])
            .u32();
        push_viewer_string(e, strings, text);
        let state = if e.mem.u8(data + 0xa2) != 0 {
            TEXT_ACTIVE
        } else {
            TEXT_INACTIVE
        };
        let text = e.call(VIEWER_STRING_TEXT, &args![NAME_MOTOR, state]).u32();
        push_viewer_string(e, strings, text);
        let bits = x87_quiet_bits(e.mem.u32(data + 0xac));
        let value = degrees_bits(e, bits);
        let text = e.call(VIEWER_STRING_FLOAT, &args![NAME_ANGLE, value]).u32();
        push_viewer_string(e, strings, text);
    }
    e.mem.set_u32(word, saved);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00caa140, bhk_limited_hinge_constraint_destruct(Ptr)),
        entry!(0x00caa1b0, fn_00caa1b0(Ptr, u32)),
        entry!(0x00caa1c0, fn_00caa1c0() -> u32),
        entry!(0x00caa250, fn_00caa250(Ptr, u32, u32)),
        entry!(0x00caa3f0, fn_00caa3f0(Ptr) -> u32),
        entry!(0x00caa4a0, bhk_limited_hinge_constraint_create_clone(Ptr, u32) -> u32),
        entry!(0x00caa5d0, bhk_limited_hinge_constraint_create_cinfo(Ptr, Ptr) -> u32),
        entry!(0x00caa6a0, fn_00caa6a0(Ptr, u32)),
        entry!(0x00caa840, fn_00caa840(Ptr, u32)),
        entry!(0x00caa960, fn_00caa960(Ptr, u32)),
    ]
}

/// Test support shared by the BSHavok unit files translated alongside this one.
#[cfg(test)]
pub(crate) mod support {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// Calls recorded by [`record`]: `(address, argument words)`.
    pub(crate) type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    pub(crate) fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// Records every call to `addr` (with its words) in `log` and answers `eax`.
    pub(crate) fn record(e: &mut Engine, addr: u32, eax: u32, log: &Calls) {
        let log = Rc::clone(log);
        e.register_double(addr, move |_, a| {
            log.borrow_mut().push((addr, a.to_vec()));
            ret(eax)
        });
    }

    /// The argument words of every recorded call to `addr`.
    pub(crate) fn calls_to(log: &Calls, addr: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A stream with a reader and a writer that copy through the doubles at `0x0900_0008`
    /// (the reader fills the buffer with bytes `0x10, 0x11, ...`) and `0x0900_000c`; the
    /// function words are recorded in `log`. The stream's bytes at `+0x1e4` and `+0x1e5`
    /// are 1 (same byte order as the file).
    pub(crate) fn stream_fixture(e: &mut Engine, log: &Calls) -> u32 {
        let io = e.mem.alloc(0x20);
        e.mem.set_u32(io + 8, 0x0900_0008);
        e.mem.set_u32(io + 0xc, 0x0900_000c);
        let reader = Rc::clone(log);
        e.register_double(0x0900_0008, move |e, a| {
            reader.borrow_mut().push((0x0900_0008, a.to_vec()));
            for i in 0..a[2] {
                e.mem.set_u8(a[1] + i, 0x10 + i as u8);
            }
            ret(a[2])
        });
        let writer = Rc::clone(log);
        e.register_double(0x0900_000c, move |_, a| {
            writer.borrow_mut().push((0x0900_000c, a.to_vec()));
            ret(a[2])
        });
        let stream = e.mem.alloc(0x300);
        e.mem.set_u32(stream + 0x24c, io);
        e.mem.set_u32(stream + 0x250, io);
        e.mem.set_u8(stream + 0x1e4, 1);
        e.mem.set_u8(stream + 0x1e5, 1);
        stream
    }

    /// The function address in slot `0x10` (free) of the allocator
    /// [`engine_with_allocator`] builds; a test that frees registers a double there.
    pub(crate) const FREE_TARGET: u32 = 0x0900_0010;

    /// An object of `size` bytes whose vtable (a zeroed 0x400-byte table) has the
    /// `(byte offset, function address)` pairs of `slots` set.
    pub(crate) fn vobject(e: &mut Engine, size: u32, slots: &[(u32, u32)]) -> u32 {
        let table = e.mem.alloc(0x400);
        for (slot, target) in slots {
            e.mem.set_u32(table + slot, *target);
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, table);
        object
    }

    /// An engine with the memory router, its allocator (slot 4 hands out the returned
    /// 0x100-byte block) and a scope word that starts at `0x77`; returns the engine, the
    /// word's address and the block.
    pub(crate) fn engine_with_allocator() -> (Engine, u32, u32) {
        let mut e = Engine::new();
        let block = e.mem.alloc(0x100);
        let heap = e.mem.alloc(0x10);
        let table = e.mem.alloc(0x20);
        e.mem.set_u32(heap, table);
        e.mem.set_u32(table + 4, 0x0900_0004);
        e.mem.set_u32(table + 0x10, FREE_TARGET);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, heap);
        e.register_double(MEMORY_ROUTER, move |_, _| ret(router));
        e.register_double(0x0900_0004, move |_, a| {
            assert_eq!(a[0], heap);
            ret(block)
        });
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 0x77);
        (e, word, block)
    }
}

#[cfg(test)]
mod tests {
    use super::support::*;
    use super::*;
    use std::rc::Rc;

    #[test]
    fn destruct_sets_the_vtable_kills_and_counts_down() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        e.set_global(COUNT_LIMITED_HINGE, 5u32);
        let log: Calls = Rc::default();
        record(&mut e, SERIALIZABLE_KILL, 0, &log);
        record(&mut e, CONSTRAINT_DESTRUCT, 0, &log);
        let this = e.mem.alloc(0x10);
        e.call(0x00ca_a140, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_LIMITED_HINGE);
        assert_eq!(e.global::<u32>(COUNT_LIMITED_HINGE), 4);
        let order: Vec<u32> = log.borrow().iter().map(|c| c.0).collect();
        assert_eq!(order, [SERIALIZABLE_KILL, CONSTRAINT_DESTRUCT]);
        assert_eq!(calls_to(&log, CONSTRAINT_DESTRUCT), [[this]]);
    }

    #[test]
    fn set_entity_b_forwards_both_words() {
        let mut e = Engine::new();
        let log: Calls = Rc::default();
        record(&mut e, CONSTRAINT_SET_ENTITY_B, 0, &log);
        e.call(0x00ca_a1b0, &args![0x1234u32, 0x5678u32]);
        assert_eq!(calls_to(&log, CONSTRAINT_SET_ENTITY_B), [[0x1234, 0x5678]]);
    }

    #[test]
    fn fn_00caa1c0_builds_the_object_and_counts_it() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        let object = e.mem.alloc(0x10);
        let log: Calls = Rc::default();
        record(&mut e, ALLOCATE, object, &log);
        record(&mut e, REF_OBJECT_CONSTRUCT, 0, &log);
        e.mem.set_u32(object + 0xc, 9);
        assert_eq!(e.call(0x00ca_a1c0, &args![]).u32(), object);
        assert_eq!(calls_to(&log, ALLOCATE), [[0x10]]);
        assert_eq!(calls_to(&log, REF_OBJECT_CONSTRUCT), [[object]]);
        assert_eq!(e.mem.u32(object), VTABLE_LIMITED_HINGE);
        assert_eq!(e.mem.u32(object + 0xc), 0);
        assert_eq!(e.global::<u32>(COUNT_CONSTRAINT), 1);
        assert_eq!(e.global::<u32>(COUNT_LIMITED_HINGE), 1);
        // A failed allocation answers 0 and constructs nothing.
        record(&mut e, ALLOCATE, 0, &log);
        assert_eq!(e.call(0x00ca_a1c0, &args![]).u32(), 0);
        assert_eq!(calls_to(&log, REF_OBJECT_CONSTRUCT).len(), 1);
    }

    /// A source data block with a recognisable value in every copied word.
    fn source_data(e: &mut Engine) -> u32 {
        let data = e.mem.alloc(DATA_SIZE);
        for word in 0..(DATA_SIZE / 4) {
            e.mem.set_u32(data + word * 4, 0x1000 + word);
        }
        e.mem.set_u32(data + 0xb0, 0);
        e.mem.set_f32(data + 0xbc, 1.5);
        e.mem.set_u32(data + 0xc4, 0x7f80_0001);
        data
    }

    #[test]
    fn clone_data_copies_the_blocks_and_the_floats() {
        let (mut e, word, _) = engine_with_allocator();
        let data = source_data(&mut e);
        let cinfo = e.mem.alloc(0x14);
        e.mem.set_u32(cinfo + 4, data);
        let target = e.mem.alloc(DATA_SIZE);
        let info = e.mem.alloc(0x20);
        e.mem.set_f32(info + 0x10, 1.0);
        let log: Calls = Rc::default();
        record(&mut e, APPLY_SCALE, 0, &log);
        record(&mut e, CINFO_CLONE_DATA, 0, &log);
        record(&mut e, DATA_CONSTRUCT, 0, &log);
        e.call(0x00ca_a250, &args![cinfo, target, info]);
        assert_eq!(
            e.mem.bytes(target + 0x20, 0x80),
            e.mem.bytes(data + 0x20, 0x80)
        );
        assert_eq!(e.mem.u32(target + 0x1c), 0);
        assert_eq!(e.mem.f32(target + 0xbc), 1.5);
        // A signalling NaN comes out quiet; +0xC8 is copied, +0xC0 is not.
        assert_eq!(e.mem.u32(target + 0xc4), 0x7fc0_0001);
        assert_eq!(e.mem.u32(target + 0xc8), e.mem.u32(data + 0xc8));
        assert_eq!(e.mem.u32(target + 0xc0), 0);
        assert!(calls_to(&log, APPLY_SCALE).is_empty());
        assert!(calls_to(&log, DATA_CONSTRUCT).is_empty());
        assert_eq!(calls_to(&log, CINFO_CLONE_DATA), [[cinfo, target, info]]);
        assert_eq!(e.mem.u32(word), 0x77);
    }

    #[test]
    fn clone_data_scales_clones_the_motor_and_allocates_a_missing_target() {
        let (mut e, word, fresh) = engine_with_allocator();
        let data = source_data(&mut e);
        let motor = e.mem.alloc(8);
        let table = e.mem.alloc(0x10);
        e.mem.set_u32(motor, table);
        e.mem.set_u32(table + 0xc, 0x0900_000c);
        let motor_clone = 0x4444u32;
        e.register_double(0x0900_000c, move |_, a| {
            assert_eq!(a, [motor]);
            ret(motor_clone)
        });
        e.mem.set_u32(data + 0xb0, motor);
        e.mem.set_u8(data + 0xa2, 3);
        let cinfo = e.mem.alloc(0x14);
        e.mem.set_u32(cinfo + 4, data);
        let info = e.mem.alloc(0x20);
        e.mem.set_f32(info + 0x10, 2.0);
        let log: Calls = Rc::default();
        record(&mut e, APPLY_SCALE, 0, &log);
        record(&mut e, CINFO_CLONE_DATA, 0, &log);
        record(&mut e, DATA_SET_MOTOR, 0, &log);
        record(&mut e, REMOVE_REFERENCE, 0, &log);
        record(&mut e, DATA_CONSTRUCT, fresh, &log);
        e.call(0x00ca_a250, &args![cinfo, 0u32, info]);
        assert_eq!(e.mem.u16(fresh + 4), 0xe0);
        assert_eq!(calls_to(&log, DATA_CONSTRUCT), [[fresh]]);
        assert_eq!(calls_to(&log, APPLY_SCALE), [[2.0f32.to_bits(), fresh]]);
        assert_eq!(calls_to(&log, DATA_SET_MOTOR), [[fresh, motor_clone]]);
        assert_eq!(calls_to(&log, REMOVE_REFERENCE), [[motor_clone]]);
        assert_eq!(e.mem.u8(fresh + 0xa2), 3);
        // A NaN scale is not 1.0, so it is applied too (quieted).
        e.mem.set_u32(info + 0x10, 0x7f80_0001);
        e.call(0x00ca_a250, &args![cinfo, fresh, info]);
        assert_eq!(calls_to(&log, APPLY_SCALE)[1], [0x7fc0_0001, fresh]);
        assert_eq!(e.mem.u32(word), 0x77);
    }

    #[test]
    fn create_data_builds_the_block_once() {
        let (mut e, word, fresh) = engine_with_allocator();
        let log: Calls = Rc::default();
        record(&mut e, DATA_CONSTRUCT, fresh, &log);
        let cinfo = e.mem.alloc(0x14);
        assert_eq!(e.call(0x00ca_a3f0, &args![cinfo]).u32(), 0xe0);
        assert_eq!(e.mem.u32(cinfo + 4), fresh);
        assert_eq!(e.mem.u16(fresh + 4), 0xe0);
        assert_eq!(calls_to(&log, DATA_CONSTRUCT), [[fresh]]);
        assert_eq!(e.mem.u32(word), 0x77);
        // Already built: nothing is allocated or constructed again.
        assert_eq!(e.call(0x00ca_a3f0, &args![cinfo]).u32(), 0xe0);
        assert_eq!(calls_to(&log, DATA_CONSTRUCT).len(), 1);
    }

    /// A limited-hinge constraint whose Havok object holds `data` at `+0xC`.
    fn constraint(e: &mut Engine, hk_object: u32, info: u32) -> u32 {
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, hk_object);
        e.mem.set_u32(this + 0xc, info);
        let table = e.mem.alloc(0xc0);
        e.mem.set_u32(this, table);
        this
    }

    #[test]
    fn create_clone_copies_members_and_clones_the_cinfo_when_there_is_a_motor() {
        let (mut e, word, _) = engine_with_allocator();
        e.map(0x0126_8000, 0x1000);
        let clone = e.mem.alloc(0x10);
        let data = e.mem.alloc(DATA_SIZE);
        e.mem.set_u32(data + 0xb0, 1);
        let hk_object = e.mem.alloc(0x20);
        e.mem.set_u32(hk_object + 0xc, data);
        let this = constraint(&mut e, hk_object, 0);
        let table = e.mem.u32(this);
        e.mem.set_u32(table + 0xb4, 0x0900_00b4);
        let cinfo_object = e.mem.alloc(0x14);
        let cinfo_table = e.mem.alloc(0x10);
        e.mem.set_u32(cinfo_object, cinfo_table);
        e.mem.set_u32(cinfo_table + 4, 0x0900_0104);
        let log: Calls = Rc::default();
        record(&mut e, CLONE_CRIT_LOCK, 0, &log);
        record(&mut e, CLONE_CRIT_UNLOCK, 0, &log);
        record(&mut e, ALLOCATE, clone, &log);
        record(&mut e, REF_OBJECT_CONSTRUCT, 0, &log);
        record(&mut e, CONSTRAINT_COPY_MEMBERS, 0, &log);
        record(&mut e, 0x0900_00b4, cinfo_object + 4, &log);
        record(&mut e, 0x0900_0104, 0, &log);
        assert_eq!(e.call(0x00ca_a4a0, &args![this, 0x99u32]).u32(), clone);
        let order: Vec<u32> = log.borrow().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            [
                CLONE_CRIT_LOCK,
                ALLOCATE,
                REF_OBJECT_CONSTRUCT,
                CONSTRAINT_COPY_MEMBERS,
                0x0900_00b4,
                0x0900_0104,
                CLONE_CRIT_UNLOCK
            ]
        );
        assert_eq!(e.mem.u32(clone), VTABLE_LIMITED_HINGE);
        assert_eq!(
            calls_to(&log, CONSTRAINT_COPY_MEMBERS),
            [[this, clone, 0x99]]
        );
        assert_eq!(calls_to(&log, 0x0900_0104), [[cinfo_object, 0, 0x99]]);
        assert_eq!(e.mem.u32(word), 0x77);
    }

    #[test]
    fn create_clone_without_a_motor_stops_after_copying_the_members() {
        let (mut e, _, _) = engine_with_allocator();
        e.map(0x0126_8000, 0x1000);
        let clone = e.mem.alloc(0x10);
        // No Havok object: the first word of pInfo is the data, which has no motor.
        let data = e.mem.alloc(DATA_SIZE);
        let info = e.mem.alloc(8);
        e.mem.set_u32(info, data);
        let this = constraint(&mut e, 0, info);
        let log: Calls = Rc::default();
        record(&mut e, CLONE_CRIT_LOCK, 0, &log);
        record(&mut e, CLONE_CRIT_UNLOCK, 0, &log);
        record(&mut e, ALLOCATE, clone, &log);
        record(&mut e, REF_OBJECT_CONSTRUCT, 0, &log);
        record(&mut e, CONSTRAINT_COPY_MEMBERS, 0, &log);
        assert_eq!(e.call(0x00ca_a4a0, &args![this, 1u32]).u32(), clone);
        assert_eq!(log.borrow().len(), 5);
        // A failed allocation hands 0 to CopyMembers and answers 0.
        record(&mut e, ALLOCATE, 0, &log);
        assert_eq!(e.call(0x00ca_a4a0, &args![this, 1u32]).u32(), 0);
        assert_eq!(calls_to(&log, CONSTRAINT_COPY_MEMBERS)[1], [this, 0, 1]);
        // Neither a Havok object nor a pInfo: nothing more to do.
        let bare = constraint(&mut e, 0, 0);
        assert_eq!(e.call(0x00ca_a4a0, &args![bare, 1u32]).u32(), 0);
    }

    #[test]
    fn create_cinfo_builds_it_once_and_hands_it_to_the_havok_object() {
        let (mut e, word, _) = engine_with_allocator();
        let block = e.mem.alloc(CINFO_SIZE);
        let log: Calls = Rc::default();
        record(&mut e, POOL_ALLOCATE, block, &log);
        record(&mut e, CONSTRAINT_COPY_CINFO, 0, &log);
        let this = constraint(&mut e, 0x5000, 0);
        let created = e.mem.alloc(4);
        e.mem.set_u32(created, 0xffff_ffff);
        assert_eq!(e.call(0x00ca_a5d0, &args![this, created]).u32(), block + 4);
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(calls_to(&log, POOL_ALLOCATE), [[POOL_OBJECT, CINFO_SIZE]]);
        assert_eq!(e.mem.u32(block), VTABLE_CINFO);
        assert_eq!(e.mem.u32(block + 4), 0);
        assert_eq!(e.mem.u32(block + 8), 1);
        assert_eq!(e.mem.u32(this + 0xc), block + 4);
        assert_eq!(calls_to(&log, CONSTRAINT_COPY_CINFO), [[this, block]]);
        assert_eq!(e.mem.u32(word), 0x77);
        // Second time: kept, not rebuilt.
        assert_eq!(e.call(0x00ca_a5d0, &args![this, created]).u32(), block + 4);
        assert_eq!(e.mem.u8(created), 0);
        assert_eq!(calls_to(&log, POOL_ALLOCATE).len(), 1);
        // No Havok object: CopyCinfo is not called; a failed allocation leaves pInfo 0.
        record(&mut e, POOL_ALLOCATE, 0, &log);
        let plain = constraint(&mut e, 0, 0);
        assert_eq!(e.call(0x00ca_a5d0, &args![plain, created]).u32(), 0);
        assert_eq!(calls_to(&log, CONSTRAINT_COPY_CINFO).len(), 1);
    }

    #[test]
    fn load_binary_reads_the_blocks_and_defaults_a_zero_friction() {
        let mut e = Engine::new();
        let log: Calls = Rc::default();
        let stream = stream_fixture(&mut e, &log);
        let io = e.mem.u32(stream + 0x24c);
        record(&mut e, CINFO_LOAD_BINARY, 0, &log);
        e.map(0x0101_6000, 0x1000);
        e.set_global(DEFAULT_MAX_FRICTION, 0x42c8_0000u32);
        let data = e.mem.alloc(DATA_SIZE);
        let cinfo = e.mem.alloc(0x14);
        e.mem.set_u32(cinfo + 4, data);
        e.mem.set_u32(stream + 4, 0x1d);
        e.call(0x00ca_a6a0, &args![cinfo, stream]);
        let reads = calls_to(&log, 0x0900_0008);
        assert_eq!(reads.len(), 5);
        assert_eq!(reads[0][..4], [io, data + 0x20, 0x40, 0]);
        assert_eq!(reads[1][..5], [io, data + 0x60, 0x40, 0, 0]);
        assert_eq!((reads[2][1], reads[2][2], reads[2][4]), (data + 0xc4, 4, 1));
        assert_eq!(reads[3][1], data + 0xc8);
        assert_eq!(reads[4][1], data + 0xbc);
        assert_eq!(e.mem.u32(io + 4), 0x40 + 0x40 + 12);
        // The bytes read land in the data; +0xBC was read as 0x13121110 (not 0).
        assert_eq!(e.mem.u32(data + 0xbc), 0x1312_1110);
        assert_eq!(calls_to(&log, CINFO_LOAD_BINARY), [[cinfo, stream]]);
        // Stream version 0x1D has no motor; a zero friction becomes the default.
        e.register_double(0x0900_0008, move |_, a| ret(a[2]));
        e.mem.set_u32(data + 0xbc, 0);
        e.call(0x00ca_a6a0, &args![cinfo, stream]);
        assert_eq!(e.mem.u32(data + 0xbc), 0x42c8_0000);
    }

    #[test]
    fn load_binary_swaps_a_foreign_stream_and_loads_the_motor() {
        let mut e = Engine::new();
        let log: Calls = Rc::default();
        let stream = stream_fixture(&mut e, &log);
        e.register_double(0x0900_0008, move |_, a| ret(a[2]));
        record(&mut e, CINFO_LOAD_BINARY, 0, &log);
        record(&mut e, FIX_ROTATION_BLOCK, 0, &log);
        record(&mut e, CINFO_LOAD_MOTOR, 0x7000, &log);
        record(&mut e, DATA_SET_MOTOR, 0, &log);
        record(&mut e, REMOVE_REFERENCE, 0, &log);
        e.mem.set_u8(stream + 0x1e5, 0);
        e.mem.set_u32(stream + 4, 0x1e);
        let data = e.mem.alloc(DATA_SIZE);
        for word in 0..4u32 {
            e.mem.set_u32(data + 0x50 + 4 * word, 0x0102_0304);
            e.mem.set_u32(data + 0x90 + 4 * word, 0x0a0b_0c0d);
        }
        e.mem.set_f32(data + 0xbc, 2.0);
        let cinfo = e.mem.alloc(0x14);
        e.mem.set_u32(cinfo + 4, data);
        e.call(0x00ca_a6a0, &args![cinfo, stream]);
        assert_eq!(e.mem.u32(data + 0x50), 0x0403_0201);
        assert_eq!(e.mem.u32(data + 0x5c), 0x0403_0201);
        assert_eq!(e.mem.u32(data + 0x90), 0x0d0c_0b0a);
        assert_eq!(
            calls_to(&log, FIX_ROTATION_BLOCK),
            [[data + 0x20], [data + 0x60]]
        );
        assert_eq!(calls_to(&log, CINFO_LOAD_MOTOR), [[cinfo, stream]]);
        assert_eq!(calls_to(&log, DATA_SET_MOTOR), [[data, 0x7000]]);
        assert_eq!(calls_to(&log, REMOVE_REFERENCE), [[0x7000]]);
        // No motor: nothing is read for +0xA2 and nothing is released.
        record(&mut e, CINFO_LOAD_MOTOR, 0, &log);
        e.call(0x00ca_a6a0, &args![cinfo, stream]);
        assert_eq!(calls_to(&log, REMOVE_REFERENCE).len(), 1);
    }

    #[test]
    fn save_binary_writes_the_blocks_and_the_active_byte_only_with_a_motor() {
        let mut e = Engine::new();
        let log: Calls = Rc::default();
        let stream = stream_fixture(&mut e, &log);
        let io = e.mem.u32(stream + 0x250);
        record(&mut e, CINFO_SAVE_BINARY, 0, &log);
        record(&mut e, CINFO_SAVE_MOTOR, 0, &log);
        let data = e.mem.alloc(DATA_SIZE);
        let cinfo = e.mem.alloc(0x14);
        e.mem.set_u32(cinfo + 4, data);
        e.call(0x00ca_a840, &args![cinfo, stream]);
        let writes = calls_to(&log, 0x0900_000c);
        let targets: Vec<(u32, u32)> = writes.iter().map(|w| (w[1] - data, w[2])).collect();
        assert_eq!(
            targets,
            [(0x20, 0x40), (0x60, 0x40), (0xc4, 4), (0xc8, 4), (0xbc, 4)]
        );
        assert_eq!(writes[0][0], io);
        assert_eq!(e.mem.u32(io + 4), 0x40 + 0x40 + 12);
        assert_eq!(calls_to(&log, CINFO_SAVE_MOTOR), [[0, stream]]);
        e.mem.set_u32(data + 0xb0, 0x7000);
        e.call(0x00ca_a840, &args![cinfo, stream]);
        let writes = calls_to(&log, 0x0900_000c);
        assert_eq!(writes.len(), 11);
        assert_eq!((writes[10][1], writes[10][2]), (data + 0xa2, 1));
        assert_eq!(calls_to(&log, CINFO_SAVE_MOTOR)[1], [0x7000, stream]);
        assert_eq!(calls_to(&log, CINFO_SAVE_BINARY).len(), 2);
    }

    #[test]
    fn get_viewer_strings_lists_the_points_angles_and_motor_state() {
        let (mut e, word, _) = engine_with_allocator();
        e.map(0x0102_f000, 0x1000);
        e.set_global(DEGREES_FACTOR, 57.0f64);
        let data = e.mem.alloc(DATA_SIZE);
        e.mem.set_f32(data + 0xc4, 2.0);
        e.mem.set_f32(data + 0xc8, 3.0);
        e.mem.set_f32(data + 0xbc, 4.0);
        e.mem.set_f32(data + 0xac, 0.5);
        e.mem.set_u8(data + 0xa2, 1);
        let cinfo = e.mem.alloc(0x14);
        e.mem.set_u32(cinfo + 4, data);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 64);
        let log: Calls = Rc::default();
        record(&mut e, CINFO_GET_VIEWER_STRINGS, 0, &log);
        record(&mut e, POINT_FROM_VECTOR, 0, &log);
        record(&mut e, POINT_GET_VIEWER_STRING, 0x3000, &log);
        record(&mut e, VIEWER_STRING_FLOAT, 0x3001, &log);
        record(&mut e, VIEWER_STRING_TEXT, 0x3002, &log);
        e.register_double(STRINGS_SET_AT, |e, a| {
            let text = e.mem.u32(a[2]);
            e.mem.set_u32(0x0900_1000 + 4 * a[1], text);
            let size = e.mem.u16(a[0] + 0xa);
            e.mem.set_u16(a[0] + 0xa, size + 1);
            Ret::default()
        });
        e.map(0x0900_1000, 0x100);
        e.call(0x00ca_a960, &args![cinfo, strings]);
        let texts: Vec<u32> = (0..10).map(|i| e.mem.u32(0x0900_1000 + 4 * i)).collect();
        assert_eq!(
            texts,
            [0x3000, 0x3000, 0x3000, 0x3000, 0x3000, 0x3001, 0x3001, 0x3001, 0x3002, 0x3001]
        );
        let sources: Vec<u32> = calls_to(&log, POINT_FROM_VECTOR)
            .iter()
            .map(|c| c[1] - data)
            .collect();
        assert_eq!(sources, [0x50, 0x90, 0x60, 0x30, 0x40]);
        let names: Vec<u32> = calls_to(&log, POINT_GET_VIEWER_STRING)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(
            names,
            [
                NAME_PIVOT_IN_A,
                NAME_PIVOT_IN_B,
                NAME_AXLE_IN_B,
                NAME_PERP_AXIS_IN_A,
                NAME_PERP_AXIS_IN_B
            ]
        );
        let floats = calls_to(&log, VIEWER_STRING_FLOAT);
        let expected = [
            (NAME_MIN_ANGLE, (2.0f64 * 57.0) as f32),
            (NAME_MAX_ANGLE, (3.0f64 * 57.0) as f32),
            (NAME_MAX_FRICTION, 4.0),
            (NAME_ANGLE, (0.5f64 * 57.0) as f32),
        ];
        for (call, (name, value)) in floats.iter().zip(expected) {
            assert_eq!(*call, [name, value.to_bits()]);
        }
        assert_eq!(
            calls_to(&log, VIEWER_STRING_TEXT),
            [[NAME_MOTOR, TEXT_ACTIVE]]
        );
        assert_eq!(e.mem.u32(word), 0x77);
        // Inactive motor; and with no data block only the base strings are added.
        e.mem.set_u8(data + 0xa2, 0);
        e.call(0x00ca_a960, &args![cinfo, strings]);
        assert_eq!(
            calls_to(&log, VIEWER_STRING_TEXT)[1],
            [NAME_MOTOR, TEXT_INACTIVE]
        );
        e.mem.set_u32(cinfo + 4, 0);
        let before = log.borrow().len();
        e.call(0x00ca_a960, &args![cinfo, strings]);
        assert_eq!(log.borrow().len(), before + 1);
        assert_eq!(e.mem.u32(word), 0x77);
    }
}
