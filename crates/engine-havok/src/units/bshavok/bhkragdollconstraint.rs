//! `BSHavok/bhkragdollconstraint.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkRagdollConstraint` (0x10 bytes, vtable `010c8bec`) holds a pointer to the Havok
//! `hkpRagdollConstraintData` (0x140 bytes) at `+0x4`/`+0xc` through its cinfo holders. The
//! data keeps, in the layout the functions here read: twist axis A `+0x20`, plane axis A
//! `+0x30`, pivot A `+0x50`, twist axis B `+0x60`, plane axis B `+0x70`, pivot B `+0x90`
//! (vectors of four floats), the motor pointer `+0xe0`, the maximum friction torque `+0xf8`,
//! the motor flag byte `+0xa2` and the angle limits `+0x104`, `+0x108`, `+0x11c`, `+0x12c`,
//! `+0x130`.
//!
//! SSE notes: the vector code computes in `f32` lane by lane in the exe's operand order;
//! `RSQRTSS` is the host CPU's estimate (the exe's own is the guest CPU's), refined once the
//! way the code does. `FLD`/`FSTP` copies of a float quiet a signalling NaN ([`x87_quiet_bits`]).
//!
//! The exception-unwinding frames (`FS:[0]` registrations) of the functions marked
//! `Translation incomplete` are not modeled.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::x87::rsqrt_estimate;

/// The memory router (`00c85750`): its `+0x10` is the allocator whose slot `+4` allocates `(bytes)`.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// Per-thread word (`TLS + 0x2b4`) the cloning and cinfo functions set to 0x10 while they allocate.
const SCOPE_WORD: u32 = 0x2b4;
/// Heap allocation (`00aa13e0`, cdecl `(size)`) and `MemoryManager::Allocate` (`00aa3e40`,
/// thiscall `(size)` on the manager `011f6238`).
const HEAP_ALLOCATE: u32 = 0x00aa_13e0;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `bhkRefObject` constructor body (`00c87c30`) and the destruction base calls.
const REF_OBJECT_CONSTRUCT: u32 = 0x00c8_7c30;
/// `bhkSerializable::Kill` (`00c876a0`) and `bhkConstraint` destructor body (`00c89640`).
const SERIALIZABLE_KILL: u32 = 0x00c8_76a0;
const CONSTRAINT_DESTRUCT: u32 = 0x00c8_9640;
/// The lock and unlock the cloning functions take (`00c876f0`, `00c87700`).
const CLONE_LOCK: u32 = 0x00c8_76f0;
const CLONE_UNLOCK: u32 = 0x00c8_7700;
/// `bhkConstraint` functions: clone members (`00c89730`, `(clone, options)`), the cinfo-holder
/// release (`00c89c80`, `(cinfo holder)`).
const CONSTRAINT_CLONE: u32 = 0x00c8_9730;
const CONSTRAINT_SET_CINFO: u32 = 0x00c8_9c80;
/// `hkConstraintCinfo` functions: clone data (`00c8a1b0`, `(data, options)`), load (`00c8a230`),
/// save (`00c8a280`), motor load (`00c8a330`), motor save (`00c8a4b0`) and viewer strings
/// (`00c8a560`).
const CINFO_CLONE_DATA: u32 = 0x00c8_a1b0;
const CINFO_LOAD_BINARY: u32 = 0x00c8_a230;
const CINFO_SAVE_BINARY: u32 = 0x00c8_a280;
const CINFO_LOAD_MOTOR: u32 = 0x00c8_a330;
const CINFO_SAVE_MOTOR: u32 = 0x00c8_a4b0;
const CINFO_GET_VIEWER_STRINGS: u32 = 0x00c8_a560;
/// `hkpRagdollConstraintData` constructor (`00cdf2f0`), `setMaxFrictionTorque` (`00cde9f0`),
/// the twist motor getter (`00cdea20`, `MOV EAX,[ECX+0xe0]`) and the three motor setters
/// (`00cdea70`, `00cdeab0`, `00cdea90`: motor index 0, 1, 2).
const DATA_CONSTRUCT: u32 = 0x00cd_f2f0;
const DATA_SET_MAX_FRICTION_TORQUE: u32 = 0x00cd_e9f0;
const DATA_GET_TWIST_MOTOR: u32 = 0x00cd_ea20;
const DATA_SET_MOTOR_0: u32 = 0x00cd_ea70;
const DATA_SET_MOTOR_1: u32 = 0x00cd_eab0;
const DATA_SET_MOTOR_2: u32 = 0x00cd_ea90;
/// `hkReferencedObject::removeReference` (`00c905b0`).
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// Live-object counters the constructors and the destructor adjust (the `bhkConstraint` one and
/// the `bhkRagdollConstraint` one), and the vtables stored in turn (RTTI-confirmed classes
/// `bhkConstraint` and `bhkRagdollConstraint`).
const COUNT_CONSTRAINT: u32 = 0x0126_8188;
const COUNT_RAGDOLL_CONSTRAINT: u32 = 0x0126_860c;
const VTABLE_CONSTRAINT: u32 = 0x0108_febc;
const VTABLE_RAGDOLL_CONSTRAINT: u32 = 0x010c_8bec;
/// Vtable of the 0x14-byte object `CreateCinfo` builds (RTTI `hkRagdollConstraintCinfo`); the
/// pointer it stores at `+0xc` is the address 4 bytes past the vtable.
const VTABLE_CINFO_BLOCK: u32 = 0x010c_8ce4;
/// Size of `hkpRagdollConstraintData`.
const DATA_SIZE: u32 = 0x140;
/// `NiPoint3` copy (`004a3970`, cdecl `(destination, source)`), `NiPoint3::GetViewerString`
/// (`00a59af0`, thiscall `(name)`) and `NiGetViewerString(char const*, float)` (`00a7d550`, cdecl).
const COPY_POINT: u32 = 0x004a_3970;
const NI_POINT3_GET_VIEWER_STRING: u32 = 0x00a5_9af0;
const NI_GET_VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
/// The viewer-string array's grow (`0096ad30`) and set-at (`0096ae90`) methods.
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// Constants the code reads: the zero the normalisation compares with, the `3.0` and `0.5`
/// of the refinement step, the `100.0` default friction torque, and radians to degrees (`double`).
const ZERO: u32 = 0x0101_1d78;
const THREE: u32 = 0x0101_7718;
const HALF: u32 = 0x0101_6248;
const DEFAULT_FRICTION_TORQUE: u32 = 0x0101_6410;
const DEGREES_PER_RADIAN: u32 = 0x0102_f248;
/// Names of the viewer strings, in the order the function appends them: the six vectors, then the
/// six floats.
const VECTOR_NAMES: [(u32, u32); 6] = [
    (0x50, 0x010c_839c),
    (0x30, 0x010c_9c74),
    (0x20, 0x010c_9c68),
    (0x90, 0x010c_8390),
    (0x70, 0x010c_9c5c),
    (0x60, 0x010c_9c50),
];
const ANGLE_NAMES: [(u32, u32); 5] = [
    (0x11c, 0x010c_9c40),
    (0x12c, 0x010c_9c30),
    (0x130, 0x010c_9c20),
    (0x104, 0x010c_9c10),
    (0x108, 0x010c_9c00),
];
const FRICTION_TORQUE_NAME: u32 = 0x010c_8340;

/// Sets the allocation-scope word to 0x10 and returns its address and old value.
fn enter_scope(e: &mut Engine) -> (u32, u32) {
    let word = e.tls() + SCOPE_WORD;
    let saved = e.mem.u32(word);
    e.mem.set_u32(word, 0x10);
    (word, saved)
}

fn increment_count(e: &mut Engine, counter: u32) {
    let value: u32 = e.global(counter);
    e.set_global(counter, value.wrapping_add(1));
}

/// Allocates `bytes` through the current Havok allocator (slot 4 of `router + 0x10`).
fn havok_allocate(e: &mut Engine, bytes: u32) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.vcall(allocator, 4, &args![bytes]).u32()
}

/// A new `hkpRagdollConstraintData`: the 0x140-byte block (its size kept in the 16-bit field at
/// `+4`) and the constructor `00cdf2f0`, whose result is the data.
fn new_constraint_data(e: &mut Engine) -> u32 {
    let block = havok_allocate(e, DATA_SIZE);
    e.mem.set_u16(block + 4, DATA_SIZE as u16);
    e.call(DATA_CONSTRUCT, &args![block]).u32()
}

/// A new constraint object as `00cc67a0` and `00cc69d0` build it: the 0x10-byte block from the
/// heap and, when it was got, the `bhkRefObject` constructor and the two vtable and count
/// steps. Answers 0 when the heap gave nothing.
fn new_constraint(e: &mut Engine) -> u32 {
    let object = e.call(HEAP_ALLOCATE, &args![0x10u32]).u32();
    if object == 0 {
        return 0;
    }
    e.call(REF_OBJECT_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, VTABLE_CONSTRAINT);
    e.mem.set_u32(object + 0xc, 0);
    increment_count(e, COUNT_CONSTRAINT);
    e.mem.set_u32(object, VTABLE_RAGDOLL_CONSTRAINT);
    increment_count(e, COUNT_RAGDOLL_CONSTRAINT);
    object
}

/// One transfer through the stream callback object at `owner + field`: the callback in the
/// object's `+slot` is called as `(stream, buffer, size, &size, 1)` and its answer is added to
/// the position at the object's `+4`.
fn stream_transfer(e: &mut Engine, owner: u32, field: u32, slot: u32, buffer: u32, size: u32) {
    let stream = e.mem.u32(owner + field);
    let callback = e.mem.u32(stream + slot);
    e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), size);
        let done = e
            .call(callback, &args![stream, buffer, size, count, 1u32])
            .u32();
        let position = e.mem.u32(stream + 4);
        e.mem.set_u32(stream + 4, position.wrapping_add(done));
    });
}

// Translated from 00cbd9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkHingeConstraintCinfo::ApplyScale` (the map's name; the body scales the vectors at `+0x50`
/// and `+0x90` of the constraint data): cdecl `(scale, data)`. Nothing happens for a null
/// `data` or a `scale` of exactly 1.0 (NaN is not 1.0). Otherwise each lane of both vectors
/// is multiplied by `scale` (`MULPS` with the splatted scale as the first operand).
pub fn hk_hinge_constraint_cinfo_apply_scale(e: &mut Engine, scale: f32, data: Ptr) {
    if data.addr() == 0 || scale == 1.0 {
        return;
    }
    for base in [0x50u32, 0x90] {
        for lane in 0..4 {
            let at = data.addr() + base + 4 * lane;
            let value = e.mem.f32(at);
            e.mem.set_f32(at, crate::sse::mul(scale, value));
        }
    }
}

// Translated from 00cc5ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRagdollConstraint::KillCinfo` (Xbox PDB): when `flag` is not 0, deletes the cinfo kept
/// at `+0xc` (an object whose header sits 4 bytes before the pointer: its virtual slot 0 is
/// called with 1) and clears the pointer.
pub fn bhk_ragdoll_constraint_kill_cinfo(e: &mut Engine, this: Ptr, flag: u8) {
    if flag == 0 {
        return;
    }
    let cinfo = e.mem.u32(this.addr() + 0xc);
    if cinfo != 0 {
        let object = cinfo.wrapping_sub(4);
        if object != 0 {
            e.vcall(object, 0, &args![1u32]);
        }
    }
    e.mem.set_u32(this.addr() + 0xc, 0);
}

// Translated from 00cc66b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc66b0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkRagdollConstraint::~bhkRagdollConstraint` (Xbox PDB): stores the class vtable, runs
/// `bhkSerializable::Kill` (`00c876a0`), decrements the live-object counter and runs the
/// `bhkConstraint` destructor body (`00c89640`).
pub fn bhk_ragdoll_constraint_destruct(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_RAGDOLL_CONSTRAINT);
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count: u32 = e.global(COUNT_RAGDOLL_CONSTRAINT);
    e.set_global(COUNT_RAGDOLL_CONSTRAINT, count.wrapping_sub(1));
    e.call(CONSTRAINT_DESTRUCT, &args![this]);
}

// Translated from 00cc6720 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc6720 - the FS:[0] exception-unwinding frame is not modeled.
/// `hkRagdollConstraintCinfo::CreateData` (Xbox PDB): when the holder has no data (`+4`), builds
/// a new `hkpRagdollConstraintData` (0x140 bytes from the Havok allocator, constructor
/// `00cdf2f0`) and stores it there. Answers 0x140.
pub fn hk_ragdoll_constraint_cinfo_create_data(e: &mut Engine, this: Ptr) -> u32 {
    if e.mem.u32(this.addr() + 4) == 0 {
        let data = new_constraint_data(e);
        e.mem.set_u32(this.addr() + 4, data);
    }
    DATA_SIZE
}

// Translated from 00cc67a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc67a0 - the FS:[0] exception-unwinding frame is not modeled.
/// The `bhkRagdollConstraint` factory (the class-table entry for `"bhkRagdollConstraint"`): a
/// 0x10-byte object from the heap, the `bhkRefObject` constructor, the two vtable and count
/// steps (`010c8bec` last), `+0xc` cleared. Answers the object, or 0 when the heap gave none.
pub fn fn_00cc67a0(e: &mut Engine) -> Ptr {
    Ptr::new(new_constraint(e))
}

// Translated from 00cc6830 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc6830 - the FS:[0] exception-unwinding frame is not modeled.
/// `hkRagdollConstraintCinfo::CloneData` (Xbox PDB): copies the constraint data of this holder
/// (`+4`) into `target` (a new one is built when `target` is 0): the eight vectors `+0x20`
/// to `+0x9f`, the five angle limits, the friction torque (through `setMaxFrictionTorque`), and,
/// when the source has a twist motor, the motor clone with its flag byte `+0xa2`. A scale in
/// `options + 0x10` other than 1.0 is applied first to the pivots. Ends with
/// `hkConstraintCinfo::CloneData(target, options)`.
pub fn hk_ragdoll_constraint_cinfo_clone_data(
    e: &mut Engine,
    this: Ptr,
    target: u32,
    options: u32,
) {
    let source = e.mem.u32(this.addr() + 4);
    let target = if target == 0 {
        new_constraint_data(e)
    } else {
        target
    };
    for offset in (0x20..0xa0).step_by(4) {
        let word = e.mem.u32(source + offset);
        e.mem.set_u32(target + offset, word);
    }
    for offset in [0x11c, 0x12c, 0x130, 0x104, 0x108] {
        let word = x87_quiet_bits(e.mem.u32(source + offset));
        e.mem.set_u32(target + offset, word);
    }
    let friction = x87_quiet_bits(e.mem.u32(source + 0xf8));
    e.call(DATA_SET_MAX_FRICTION_TORQUE, &args![target, friction]);
    let scale = x87_quiet(f32::from_bits(e.mem.u32(options + 0x10)));
    if scale != 1.0 {
        hk_hinge_constraint_cinfo_apply_scale(e, scale, Ptr::new(target));
    }
    let motor = e.call(DATA_GET_TWIST_MOTOR, &args![source]).u32();
    if motor != 0 {
        let copy = e.vcall(motor, 0xc, &args![]).u32();
        e.call(DATA_SET_MOTOR_0, &args![target, copy]);
        e.call(DATA_SET_MOTOR_1, &args![target, copy]);
        e.call(DATA_SET_MOTOR_2, &args![target, copy]);
        e.call(REMOVE_REFERENCE, &args![copy]);
        let flag = e.mem.u8(source + 0xa2);
        e.mem.set_u8(target + 0xa2, flag);
    }
    e.call(CINFO_CLONE_DATA, &args![this, target, options]);
}

// Translated from 00cc69d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc69d0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkRagdollConstraint::CreateClone` (Xbox PDB): with the allocation-scope word at 0x10 and
/// between the clone lock and unlock (`00c876f0`, `00c87700`), builds a new constraint object
/// as the factory does and, when the heap gave one, runs `bhkConstraint`'s clone
/// (`00c89730`, `(this, clone, options)`) on it. Answers the clone (0 when none).
pub fn bhk_ragdoll_constraint_create_clone(e: &mut Engine, this: Ptr, options: u32) -> Ptr {
    let (word, saved) = enter_scope(e);
    e.call(CLONE_LOCK, &args![]);
    let clone = new_constraint(e);
    e.call(CONSTRAINT_CLONE, &args![this, clone, options]);
    e.call(CLONE_UNLOCK, &args![]);
    e.mem.set_u32(word, saved);
    Ptr::new(clone)
}

// Translated from 00cc6ab0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc6ab0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkRagdollConstraint::CreateCinfo` (Xbox PDB): with the allocation-scope word at 0x10, writes
/// whether the cinfo (`+0xc`) is missing to the byte `created`; when it is, builds a 0x14-byte
/// holder (`MemoryManager::Allocate`; the holder is at `block + 4`, with `block + 8 = 1`,
/// `block + 0xc = block + 0x10 = 0`, vtable `010c8ce4` at the block and `block + 4 = 0`),
/// stores it at `+0xc` and, when `+8` is set, hands the block to `00c89c80`. Answers `+0xc`.
pub fn bhk_ragdoll_constraint_create_cinfo(e: &mut Engine, this: Ptr, created: Ptr) -> Ptr {
    let (word, saved) = enter_scope(e);
    let missing = e.mem.u32(this.addr() + 0xc) == 0;
    e.mem.set_u8(created.addr(), missing as u8);
    if missing {
        let block = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x14u32])
            .u32();
        let holder = if block != 0 {
            e.mem.set_u32(block + 8, 1);
            e.mem.set_u32(block + 0xc, 0);
            e.mem.set_u32(block + 0x10, 0);
            e.mem.set_u32(block, VTABLE_CINFO_BLOCK);
            e.mem.set_u32(block + 4, 0);
            block + 4
        } else {
            0
        };
        e.mem.set_u32(this.addr() + 0xc, holder);
        if e.mem.u32(this.addr() + 8) != 0 {
            let argument = if holder != 0 { holder - 4 } else { 0 };
            e.call(CONSTRAINT_SET_CINFO, &args![this, argument]);
        }
    }
    let cinfo = e.mem.u32(this.addr() + 0xc);
    e.mem.set_u32(word, saved);
    Ptr::new(cinfo)
}

/// The vector at `at` scaled to unit length the way `00cc6b80` does it: the sum of squares of
/// the first three lanes as `l2 + (l1 + l0)`, `RSQRTSS` refined once
/// (`(half * r) * (three - (sum * r) * r)`), 0 when the sum equals `zero`; all four lanes are
/// multiplied by that factor (`MULPS` with the splatted factor first).
fn normalise_vector(e: &mut Engine, at: u32, zero: f32, three: f32, half: f32) {
    let lanes = [
        e.mem.f32(at),
        e.mem.f32(at + 4),
        e.mem.f32(at + 8),
        e.mem.f32(at + 12),
    ];
    let squares = lanes.map(|lane| lane * lane);
    let sum = squares[2] + (squares[1] + squares[0]);
    let estimate = rsqrt_estimate(sum);
    let step = three - (sum * estimate) * estimate;
    let factor = if sum == zero {
        0.0
    } else {
        (half * estimate) * step
    };
    for (index, lane) in lanes.iter().enumerate() {
        e.mem.set_f32(at + 4 * index as u32, factor * lane);
    }
}

// Translated from 00cc6b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkRagdollConstraintCinfo::LoadBinary` (Xbox PDB): after `hkConstraintCinfo::LoadBinary`,
/// reads through the stream callback at `reader + 0x24c` (slot `+8`) into the constraint data
/// (`+4`): the two 0x40-byte blocks at `+0x20` and `+0x60`, then the floats `+0x11c`, `+0x12c`,
/// `+0x130`, `+0x104`, `+0x108`, `+0xf8`. A friction torque of exactly 0 becomes the exe's
/// default (100.0). The axes `+0x30`, `+0x20`, `+0x70`, `+0x60` are normalised. From stream
/// version 0x1e on, the motor (`00c8a330`) is set as all three motors and, when there is one,
/// its flag byte `+0xa2` is read and the motor released.
pub fn hk_ragdoll_constraint_cinfo_load_binary(e: &mut Engine, this: Ptr, reader: Ptr) {
    e.call(CINFO_LOAD_BINARY, &args![this, reader]);
    let data = e.mem.u32(this.addr() + 4);
    let owner = reader.addr();
    for (offset, size) in [
        (0x20, 0x40),
        (0x60, 0x40),
        (0x11c, 4),
        (0x12c, 4),
        (0x130, 4),
        (0x104, 4),
        (0x108, 4),
        (0xf8, 4),
    ] {
        stream_transfer(e, owner, 0x24c, 8, data + offset, size);
    }
    if e.mem.f32(data + 0xf8) == 0.0 {
        let torque = x87_quiet_bits(e.global::<u32>(DEFAULT_FRICTION_TORQUE));
        e.mem.set_u32(data + 0xf8, torque);
    }
    let zero: f32 = e.global(ZERO);
    let three: f32 = e.global(THREE);
    let half: f32 = e.global(HALF);
    for offset in [0x30, 0x20, 0x70, 0x60] {
        normalise_vector(e, data + offset, zero, three, half);
    }
    if e.mem.u32(owner + 4) > 0x1d {
        let motor = e.call(CINFO_LOAD_MOTOR, &args![this, reader]).u32();
        e.call(DATA_SET_MOTOR_0, &args![data, motor]);
        e.call(DATA_SET_MOTOR_1, &args![data, motor]);
        e.call(DATA_SET_MOTOR_2, &args![data, motor]);
        if motor != 0 {
            stream_transfer(e, owner, 0x24c, 8, data + 0xa2, 1);
            e.call(REMOVE_REFERENCE, &args![motor]);
        }
    }
}

// Translated from 00cc6ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkRagdollConstraintCinfo::SaveBinary` (Xbox PDB): after `hkConstraintCinfo::SaveBinary`,
/// writes through the stream callback at `writer + 0x250` (slot `+0xc`) the same fields
/// `LoadBinary` reads, in the same order, then saves the motor at `+0xe0` (`00c8a4b0`) and,
/// when there is one, writes its flag byte `+0xa2`.
pub fn hk_ragdoll_constraint_cinfo_save_binary(e: &mut Engine, this: Ptr, writer: Ptr) {
    e.call(CINFO_SAVE_BINARY, &args![this, writer]);
    let data = e.mem.u32(this.addr() + 4);
    for (offset, size) in [
        (0x20, 0x40),
        (0x60, 0x40),
        (0x11c, 4),
        (0x12c, 4),
        (0x130, 4),
        (0x104, 4),
        (0x108, 4),
        (0xf8, 4),
    ] {
        stream_transfer(e, writer.addr(), 0x250, 0xc, data + offset, size);
    }
    let motor = e.mem.u32(data + 0xe0);
    // `00c8a4b0` is stdcall (RET 8, `(motor, stream)`) and does not read the ECX (this)
    // the caller sets.
    e.call(CINFO_SAVE_MOTOR, &args![motor, writer]);
    if e.mem.u32(data + 0xe0) != 0 {
        stream_transfer(e, writer.addr(), 0x250, 0xc, data + 0xa2, 1);
    }
}

/// Appends the viewer string `text` to the array `strings` (size and capacity are 16-bit fields
/// at `+0xA` and `+8`; `+0xE` is the grow step): grows it when full, then sets the element at
/// the old size.
fn push_viewer_string(e: &mut Engine, strings: u32, text: u32) {
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

// Translated from 00cc7090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkRagdollConstraintCinfo::GetViewerStrings` (Xbox PDB): after
/// `hkConstraintCinfo::GetViewerStrings`, and when there is constraint data (`+4`), appends
/// the viewer strings of the pivot, plane axis and twist axis vectors of A and B (`NiPoint3`
/// copies shown with `NiPoint3::GetViewerString`), of the five angle limits in degrees (the
/// radian value times the `double` `57.29578`, stored as a `float`) and of the friction torque
/// (copied unchanged).
pub fn hk_ragdoll_constraint_cinfo_get_viewer_strings(e: &mut Engine, this: Ptr, strings: u32) {
    e.call(CINFO_GET_VIEWER_STRINGS, &args![this, strings]);
    let data = e.mem.u32(this.addr() + 4);
    if data == 0 {
        return;
    }
    e.with_stack(12, |e, point| {
        for (offset, name) in VECTOR_NAMES {
            e.call(COPY_POINT, &args![point, data + offset]);
            let text = e
                .call(NI_POINT3_GET_VIEWER_STRING, &args![point, name])
                .u32();
            push_viewer_string(e, strings, text);
        }
    });
    let degrees: f64 = e.global(DEGREES_PER_RADIAN);
    for (offset, name) in ANGLE_NAMES {
        let radians = f64::from(e.mem.f32(data + offset));
        let shown = x87_quiet((radians * degrees) as f32);
        let text = e
            .call(NI_GET_VIEWER_STRING_FLOAT, &args![name, shown.to_bits()])
            .u32();
        push_viewer_string(e, strings, text);
    }
    let torque = x87_quiet_bits(e.mem.u32(data + 0xf8));
    let text = e
        .call(
            NI_GET_VIEWER_STRING_FLOAT,
            &args![FRICTION_TORQUE_NAME, torque],
        )
        .u32();
    push_viewer_string(e, strings, text);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cbd9e0, hk_hinge_constraint_cinfo_apply_scale(f32, Ptr)),
        entry!(0x00cc5ee0, bhk_ragdoll_constraint_kill_cinfo(Ptr, u8)),
        entry!(0x00cc66b0, bhk_ragdoll_constraint_destruct(Ptr)),
        entry!(0x00cc6720, hk_ragdoll_constraint_cinfo_create_data(Ptr) -> u32),
        entry!(0x00cc67a0, fn_00cc67a0() -> Ptr),
        entry!(
            0x00cc6830,
            hk_ragdoll_constraint_cinfo_clone_data(Ptr, u32, u32)
        ),
        entry!(0x00cc69d0, bhk_ragdoll_constraint_create_clone(Ptr, u32) -> Ptr),
        entry!(0x00cc6ab0, bhk_ragdoll_constraint_create_cinfo(Ptr, Ptr) -> Ptr),
        entry!(
            0x00cc6b80,
            hk_ragdoll_constraint_cinfo_load_binary(Ptr, Ptr)
        ),
        entry!(
            0x00cc6ef0,
            hk_ragdoll_constraint_cinfo_save_binary(Ptr, Ptr)
        ),
        entry!(
            0x00cc7090,
            hk_ragdoll_constraint_cinfo_get_viewer_strings(Ptr, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const ALLOCATE_SLOT_DOUBLE: u32 = 0x0900_0004;
    const READ_DOUBLE: u32 = 0x0900_0010;
    const WRITE_DOUBLE: u32 = 0x0900_0020;

    /// An engine with the globals area mapped and a logging `ret 0` double over every address
    /// in `quiet`.
    fn engine(quiet: &[u32]) -> (Engine, Log) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        for &address in quiet {
            returning(&mut e, &log, address, 0);
        }
        (e, log)
    }

    /// A logging double over `address` that answers `value` in `EAX`.
    fn returning(e: &mut Engine, log: &Log, address: u32, value: u32) {
        let kept = Rc::clone(log);
        e.register_double(address, move |_, a| {
            kept.borrow_mut().push((address, a.to_vec()));
            Ret {
                eax: value,
                ..Ret::default()
            }
        });
    }

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn order(log: &Log) -> Vec<u32> {
        log.borrow().iter().map(|c| c.0).collect()
    }

    /// A memory router whose allocator hands out zeroed blocks through slot 4 (logged).
    fn install_allocator(e: &mut Engine, log: &Log) {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        returning(e, log, MEMORY_ROUTER, router);
        let kept = Rc::clone(log);
        e.register_double(ALLOCATE_SLOT_DOUBLE, move |e, a| {
            kept.borrow_mut().push((ALLOCATE_SLOT_DOUBLE, a.to_vec()));
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
    }

    /// A stream callback object (position at `+4`, read callback at `+8`, write callback at
    /// `+0xc`) shared by a reader (`+0x24c`) and a writer (`+0x250`) owner.
    fn stream_owner(e: &mut Engine, version: u32) -> (u32, u32) {
        let stream = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 8, READ_DOUBLE);
        e.mem.set_u32(stream + 0xc, WRITE_DOUBLE);
        let owner = e.mem.alloc(0x260);
        e.mem.set_u32(owner + 4, version);
        e.mem.set_u32(owner + 0x24c, stream);
        e.mem.set_u32(owner + 0x250, stream);
        (owner, stream)
    }

    /// Installs a read callback that copies from `input` and answers the bytes copied; the
    /// size pointer it is handed must hold the size.
    fn install_reader(e: &mut Engine, input: Vec<u8>) {
        let input = RefCell::new(std::collections::VecDeque::from(input));
        e.register_double(READ_DOUBLE, move |e, a| {
            assert_eq!(e.mem.u32(a[3]), a[2]);
            assert_eq!(a[4], 1);
            for offset in 0..a[2] {
                let byte = input.borrow_mut().pop_front().unwrap();
                e.mem.set_u8(a[1] + offset, byte);
            }
            Ret {
                eax: a[2],
                ..Ret::default()
            }
        });
    }

    fn float_bytes(values: &[f32]) -> Vec<u8> {
        values.iter().flat_map(|v| v.to_le_bytes()).collect()
    }

    fn set_normalise_constants(e: &mut Engine) {
        e.set_global(ZERO, 0.0f32);
        e.set_global(THREE, 3.0f32);
        e.set_global(HALF, 0.5f32);
        e.set_global(DEFAULT_FRICTION_TORQUE, 100.0f32);
    }

    #[test]
    fn apply_scale_multiplies_both_pivots_unless_the_scale_is_one() {
        let (mut e, _) = engine(&[]);
        let data = e.mem.alloc(0xa0);
        for lane in 0..4 {
            e.mem.set_f32(data + 0x50 + 4 * lane, 1.0 + lane as f32);
            e.mem.set_f32(data + 0x90 + 4 * lane, -2.0);
            e.mem.set_f32(data + 0x20 + 4 * lane, 7.0);
        }
        e.call(0x00cbd9e0, &args![1.0f32, data]);
        assert_eq!(e.mem.f32(data + 0x58), 3.0);
        e.call(0x00cbd9e0, &args![0.5f32, 0u32]);
        assert_eq!(e.mem.f32(data + 0x58), 3.0);
        e.call(0x00cbd9e0, &args![2.0f32, data]);
        assert_eq!(e.mem.f32(data + 0x50), 2.0);
        assert_eq!(e.mem.f32(data + 0x5c), 8.0);
        assert_eq!(e.mem.f32(data + 0x98), -4.0);
        assert_eq!(e.mem.f32(data + 0x20), 7.0);
        e.mem.set_f32(data + 0x50, 1.0);
        e.call(0x00cbd9e0, &args![f32::NAN, data]);
        assert!(e.mem.f32(data + 0x50).is_nan());
    }

    #[test]
    fn kill_cinfo_deletes_the_cinfo_only_with_the_flag() {
        let (mut e, log) = engine(&[0x0900_0000]);
        let object = e.mem.alloc(0x20);
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable, 0x0900_0000);
        e.mem.set_u32(object, vtable);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, object + 4);
        e.call(0x00cc5ee0, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + 0xc), object + 4);
        assert!(log.borrow().is_empty());
        e.call(0x00cc5ee0, &args![this, 1u32]);
        assert_eq!(calls_to(&log, 0x0900_0000), vec![vec![object, 1]]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        e.call(0x00cc5ee0, &args![this, 1u32]);
        assert_eq!(log.borrow().len(), 1);
    }

    #[test]
    fn destructor_kills_decrements_and_runs_the_base() {
        let (mut e, log) = engine(&[SERIALIZABLE_KILL, CONSTRAINT_DESTRUCT]);
        e.set_global(COUNT_RAGDOLL_CONSTRAINT, 5u32);
        let this = e.mem.alloc(0x10);
        e.call(0x00cc66b0, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_RAGDOLL_CONSTRAINT);
        assert_eq!(e.global::<u32>(COUNT_RAGDOLL_CONSTRAINT), 4);
        assert_eq!(order(&log), vec![SERIALIZABLE_KILL, CONSTRAINT_DESTRUCT]);
        assert_eq!(calls_to(&log, CONSTRAINT_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn create_data_builds_the_data_once() {
        let (mut e, log) = engine(&[]);
        install_allocator(&mut e, &log);
        let kept = Rc::clone(&log);
        e.register_double(DATA_CONSTRUCT, move |_, a| {
            kept.borrow_mut().push((DATA_CONSTRUCT, a.to_vec()));
            Ret {
                eax: a[0] + 0x10,
                ..Ret::default()
            }
        });
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00cc6720, &args![this]).u32(), 0x140);
        let built = calls_to(&log, DATA_CONSTRUCT);
        assert_eq!(built.len(), 1);
        assert_eq!(e.mem.u32(this + 4), built[0][0] + 0x10);
        assert_eq!(e.mem.u16(built[0][0] + 4), 0x140);
        assert_eq!(calls_to(&log, ALLOCATE_SLOT_DOUBLE)[0][1], 0x140);
        assert_eq!(e.call(0x00cc6720, &args![this]).u32(), 0x140);
        assert_eq!(calls_to(&log, DATA_CONSTRUCT).len(), 1);
    }

    #[test]
    fn factory_builds_the_object_or_answers_null() {
        let (mut e, log) = engine(&[REF_OBJECT_CONSTRUCT]);
        let block = e.mem.alloc(0x10);
        returning(&mut e, &log, HEAP_ALLOCATE, block);
        e.set_global(COUNT_CONSTRAINT, 7u32);
        e.set_global(COUNT_RAGDOLL_CONSTRAINT, 9u32);
        e.mem.set_u32(block + 0xc, 0x1234);
        assert_eq!(e.call(0x00cc67a0, &args![]).u32(), block);
        assert_eq!(calls_to(&log, HEAP_ALLOCATE), vec![vec![0x10]]);
        assert_eq!(calls_to(&log, REF_OBJECT_CONSTRUCT), vec![vec![block]]);
        assert_eq!(e.mem.u32(block), VTABLE_RAGDOLL_CONSTRAINT);
        assert_eq!(e.mem.u32(block + 0xc), 0);
        assert_eq!(e.global::<u32>(COUNT_CONSTRAINT), 8);
        assert_eq!(e.global::<u32>(COUNT_RAGDOLL_CONSTRAINT), 10);
        returning(&mut e, &log, HEAP_ALLOCATE, 0);
        assert_eq!(e.call(0x00cc67a0, &args![]).u32(), 0);
        assert_eq!(calls_to(&log, REF_OBJECT_CONSTRUCT).len(), 1);
        assert_eq!(e.global::<u32>(COUNT_RAGDOLL_CONSTRAINT), 10);
    }

    /// Source data with every word of `0x20..0xa0` distinct, angle limits, a signalling-NaN
    /// friction torque and a motor flag byte.
    fn source_data(e: &mut Engine) -> u32 {
        let data = e.mem.alloc(0x140);
        for offset in (0x20..0xa0).step_by(4) {
            e.mem.set_f32(data + offset, 0.25 * offset as f32);
        }
        for (index, offset) in [0x11cu32, 0x12c, 0x130, 0x104, 0x108].iter().enumerate() {
            e.mem.set_f32(data + offset, 10.0 + index as f32);
        }
        e.mem.set_u32(data + 0xf8, 0x7f80_0001);
        e.mem.set_u8(data + 0xa2, 0x5a);
        data
    }

    #[test]
    fn clone_data_copies_scales_and_clones_the_motor() {
        let (mut e, log) = engine(&[CINFO_CLONE_DATA, REMOVE_REFERENCE]);
        for address in [
            DATA_SET_MAX_FRICTION_TORQUE,
            DATA_SET_MOTOR_0,
            DATA_SET_MOTOR_1,
            DATA_SET_MOTOR_2,
        ] {
            returning(&mut e, &log, address, 0);
        }
        let source = source_data(&mut e);
        let motor_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(motor_vtable + 0xc, 0x0900_0040);
        let motor = e.mem.alloc(0x10);
        e.mem.set_u32(motor, motor_vtable);
        returning(&mut e, &log, 0x0900_0040, 0x7000);
        returning(&mut e, &log, DATA_GET_TWIST_MOTOR, motor);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, source);
        let target = e.mem.alloc(0x140);
        e.mem.set_u8(target + 0xa2, 0);
        let options = e.mem.alloc(0x20);
        e.mem.set_f32(options + 0x10, 2.0);
        e.call(0x00cc6830, &args![this, target, options]);
        assert_eq!(e.mem.f32(target + 0x20), 8.0);
        assert_eq!(e.mem.f32(target + 0x30), 12.0);
        assert_eq!(e.mem.f32(target + 0x50), 2.0 * 0.25 * 0x50 as f32);
        assert_eq!(e.mem.f32(target + 0x9c), 2.0 * 0.25 * 0x9c as f32);
        assert_eq!(e.mem.f32(target + 0x8c), 0.25 * 0x8c as f32);
        assert_eq!(e.mem.f32(target + 0x11c), 10.0);
        assert_eq!(e.mem.f32(target + 0x108), 14.0);
        assert_eq!(calls_to(&log, DATA_GET_TWIST_MOTOR), vec![vec![source]]);
        assert_eq!(
            calls_to(&log, DATA_SET_MAX_FRICTION_TORQUE),
            vec![vec![target, 0x7fc0_0001]]
        );
        for address in [DATA_SET_MOTOR_0, DATA_SET_MOTOR_1, DATA_SET_MOTOR_2] {
            assert_eq!(calls_to(&log, address), vec![vec![target, 0x7000]]);
        }
        assert_eq!(calls_to(&log, 0x0900_0040), vec![vec![motor]]);
        assert_eq!(calls_to(&log, REMOVE_REFERENCE), vec![vec![0x7000]]);
        assert_eq!(e.mem.u8(target + 0xa2), 0x5a);
        assert_eq!(
            calls_to(&log, CINFO_CLONE_DATA),
            vec![vec![this, target, options]]
        );
    }

    #[test]
    fn clone_data_builds_a_target_and_skips_unit_scale_and_missing_motor() {
        let (mut e, log) = engine(&[CINFO_CLONE_DATA, DATA_SET_MAX_FRICTION_TORQUE]);
        install_allocator(&mut e, &log);
        let kept = Rc::clone(&log);
        e.register_double(DATA_CONSTRUCT, move |_, a| {
            kept.borrow_mut().push((DATA_CONSTRUCT, a.to_vec()));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        for address in [DATA_GET_TWIST_MOTOR, DATA_SET_MOTOR_0, REMOVE_REFERENCE] {
            returning(&mut e, &log, address, 0);
        }
        let source = source_data(&mut e);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, source);
        let options = e.mem.alloc(0x20);
        e.mem.set_f32(options + 0x10, 1.0);
        e.call(0x00cc6830, &args![this, 0u32, options]);
        let built = calls_to(&log, DATA_CONSTRUCT)[0][0];
        assert_eq!(e.mem.f32(built + 0x50), 0.25 * 0x50 as f32);
        assert_eq!(e.mem.u8(built + 0xa2), 0);
        assert!(calls_to(&log, DATA_SET_MOTOR_0).is_empty());
        assert!(calls_to(&log, REMOVE_REFERENCE).is_empty());
        assert_eq!(
            calls_to(&log, CINFO_CLONE_DATA),
            vec![vec![this, built, options]]
        );
    }

    #[test]
    fn create_clone_runs_between_the_lock_and_unlock_in_the_scope() {
        let (mut e, log) = engine(&[
            CLONE_LOCK,
            CLONE_UNLOCK,
            REF_OBJECT_CONSTRUCT,
            CONSTRAINT_CLONE,
        ]);
        let block = e.mem.alloc(0x10);
        e.register_double(HEAP_ALLOCATE, move |e, _| {
            let word = e.tls() + SCOPE_WORD;
            assert_eq!(e.mem.u32(word), 0x10);
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        e.set_global(COUNT_RAGDOLL_CONSTRAINT, 0u32);
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 0x77);
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00cc69d0, &args![this, 0x55u32]).u32(), block);
        assert_eq!(
            order(&log),
            vec![
                CLONE_LOCK,
                REF_OBJECT_CONSTRUCT,
                CONSTRAINT_CLONE,
                CLONE_UNLOCK
            ]
        );
        assert_eq!(
            calls_to(&log, CONSTRAINT_CLONE),
            vec![vec![this, block, 0x55]]
        );
        assert_eq!(e.mem.u32(block), VTABLE_RAGDOLL_CONSTRAINT);
        assert_eq!(e.global::<u32>(COUNT_RAGDOLL_CONSTRAINT), 1);
        assert_eq!(e.mem.u32(word), 0x77);
        returning(&mut e, &log, HEAP_ALLOCATE, 0);
        log.borrow_mut().clear();
        assert_eq!(e.call(0x00cc69d0, &args![this, 0x66u32]).u32(), 0);
        assert_eq!(calls_to(&log, CONSTRAINT_CLONE), vec![vec![this, 0, 0x66]]);
    }

    #[test]
    fn create_cinfo_builds_the_holder_when_it_is_missing() {
        let (mut e, log) = engine(&[CONSTRAINT_SET_CINFO]);
        let block = e.mem.alloc(0x14);
        e.register_double(MEMORY_ALLOCATE, move |e, a| {
            let word = e.tls() + SCOPE_WORD;
            assert_eq!(e.mem.u32(word), 0x10);
            assert_eq!(a, [MEMORY_MANAGER, 0x14]);
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, 0x1234);
        let flag = e.mem.alloc(4);
        e.mem.set_u8(flag, 0x99);
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 3);
        assert_eq!(e.call(0x00cc6ab0, &args![this, flag]).u32(), block + 4);
        assert_eq!(e.mem.u8(flag), 1);
        assert_eq!(e.mem.u32(this + 0xc), block + 4);
        assert_eq!(e.mem.u32(block), VTABLE_CINFO_BLOCK);
        assert_eq!(e.mem.u32(block + 4), 0);
        assert_eq!(e.mem.u32(block + 8), 1);
        assert_eq!(e.mem.u32(block + 0xc), 0);
        assert_eq!(e.mem.u32(block + 0x10), 0);
        assert_eq!(
            calls_to(&log, CONSTRAINT_SET_CINFO),
            vec![vec![this, block]]
        );
        assert_eq!(e.mem.u32(word), 3);
        // Present already: the flag is 0 and nothing is built.
        assert_eq!(e.call(0x00cc6ab0, &args![this, flag]).u32(), block + 4);
        assert_eq!(e.mem.u8(flag), 0);
        assert_eq!(calls_to(&log, CONSTRAINT_SET_CINFO).len(), 1);
    }

    #[test]
    fn create_cinfo_without_memory_stores_null_and_still_notifies() {
        let (mut e, log) = engine(&[CONSTRAINT_SET_CINFO, MEMORY_ALLOCATE]);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, 0x1234);
        let flag = e.mem.alloc(4);
        assert_eq!(e.call(0x00cc6ab0, &args![this, flag]).u32(), 0);
        assert_eq!(e.mem.u8(flag), 1);
        assert_eq!(calls_to(&log, CONSTRAINT_SET_CINFO), vec![vec![this, 0]]);
        // `+8` clear skips the notification.
        e.mem.set_u32(this + 8, 0);
        e.call(0x00cc6ab0, &args![this, flag]);
        assert_eq!(calls_to(&log, CONSTRAINT_SET_CINFO).len(), 1);
    }

    #[test]
    fn load_binary_reads_defaults_the_torque_normalises_and_loads_the_motor() {
        let (mut e, log) = engine(&[CINFO_LOAD_BINARY]);
        for address in [
            DATA_SET_MOTOR_0,
            DATA_SET_MOTOR_1,
            DATA_SET_MOTOR_2,
            REMOVE_REFERENCE,
        ] {
            returning(&mut e, &log, address, 0);
        }
        returning(&mut e, &log, CINFO_LOAD_MOTOR, 0x7000);
        set_normalise_constants(&mut e);
        let mut first = vec![0.0f32; 16];
        first[0] = 2.0; // twist axis A (0x20): length 2
        first[4 + 1] = 4.0; // plane axis A (0x30): length 4
        first[4 + 3] = 8.0; // its w lane is scaled too
        let mut second = vec![0.0f32; 16];
        second[0] = 3.0; // twist axis B (0x60)
        second[4 + 3] = 5.0; // plane axis B (0x70): zero length, so everything becomes 0
        let mut input = float_bytes(&first);
        input.extend(float_bytes(&second));
        input.extend(float_bytes(&[1.0, 2.0, 3.0, 4.0, 5.0, 0.0]));
        input.push(0x42);
        install_reader(&mut e, input);
        let (reader, stream) = stream_owner(&mut e, 0x1e);
        let data = e.mem.alloc(0x140);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, data);
        e.call(0x00cc6b80, &args![this, reader]);
        let close = |a: f32, b: f32| (a - b).abs() < 1e-3;
        assert!(close(e.mem.f32(data + 0x20), 1.0));
        assert!(close(e.mem.f32(data + 0x34), 1.0));
        assert!(close(e.mem.f32(data + 0x3c), 2.0));
        assert!(close(e.mem.f32(data + 0x60), 1.0));
        assert_eq!(e.mem.f32(data + 0x70), 0.0);
        assert_eq!(e.mem.f32(data + 0x7c), 0.0);
        assert_eq!(e.mem.f32(data + 0x11c), 1.0);
        assert_eq!(e.mem.f32(data + 0x108), 5.0);
        assert_eq!(e.mem.f32(data + 0xf8), 100.0);
        assert_eq!(e.mem.u8(data + 0xa2), 0x42);
        assert_eq!(e.mem.u32(stream + 4), 0x40 + 0x40 + 24 + 1);
        assert_eq!(calls_to(&log, CINFO_LOAD_BINARY), vec![vec![this, reader]]);
        assert_eq!(calls_to(&log, CINFO_LOAD_MOTOR), vec![vec![this, reader]]);
        for address in [DATA_SET_MOTOR_0, DATA_SET_MOTOR_1, DATA_SET_MOTOR_2] {
            assert_eq!(calls_to(&log, address), vec![vec![data, 0x7000]]);
        }
        assert_eq!(calls_to(&log, REMOVE_REFERENCE), vec![vec![0x7000]]);
    }

    #[test]
    fn load_binary_keeps_a_stored_torque_and_skips_the_motor_in_old_streams() {
        let (mut e, log) = engine(&[CINFO_LOAD_BINARY, CINFO_LOAD_MOTOR, REMOVE_REFERENCE]);
        set_normalise_constants(&mut e);
        let mut input = float_bytes(&[0.0; 32]);
        input.extend(float_bytes(&[0.0, 0.0, 0.0, 0.0, 0.0, 7.5]));
        install_reader(&mut e, input);
        let (reader, _) = stream_owner(&mut e, 0x1d);
        let data = e.mem.alloc(0x140);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, data);
        e.call(0x00cc6b80, &args![this, reader]);
        assert_eq!(e.mem.f32(data + 0xf8), 7.5);
        assert!(calls_to(&log, CINFO_LOAD_MOTOR).is_empty());
        assert!(calls_to(&log, REMOVE_REFERENCE).is_empty());
    }

    #[test]
    fn load_binary_without_a_motor_reads_no_flag_byte() {
        let (mut e, log) = engine(&[CINFO_LOAD_BINARY, REMOVE_REFERENCE]);
        for address in [DATA_SET_MOTOR_0, DATA_SET_MOTOR_1, DATA_SET_MOTOR_2] {
            returning(&mut e, &log, address, 0);
        }
        returning(&mut e, &log, CINFO_LOAD_MOTOR, 0);
        set_normalise_constants(&mut e);
        let mut input = float_bytes(&[0.0; 32]);
        input.extend(float_bytes(&[0.0, 0.0, 0.0, 0.0, 0.0, 1.0]));
        install_reader(&mut e, input);
        let (reader, stream) = stream_owner(&mut e, 0x20);
        let data = e.mem.alloc(0x140);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, data);
        e.call(0x00cc6b80, &args![this, reader]);
        assert_eq!(e.mem.u32(stream + 4), 0x80 + 24);
        assert_eq!(calls_to(&log, DATA_SET_MOTOR_2), vec![vec![data, 0]]);
        assert!(calls_to(&log, REMOVE_REFERENCE).is_empty());
    }

    #[test]
    fn save_binary_writes_the_fields_in_order_and_the_motor_flag() {
        let (mut e, log) = engine(&[CINFO_SAVE_BINARY, CINFO_SAVE_MOTOR]);
        let written: Rc<RefCell<Vec<(u32, Vec<u8>)>>> = Rc::new(RefCell::new(Vec::new()));
        let kept = Rc::clone(&written);
        e.register_double(WRITE_DOUBLE, move |e, a| {
            assert_eq!(e.mem.u32(a[3]), a[2]);
            assert_eq!(a[4], 1);
            kept.borrow_mut().push((a[1], e.mem.bytes(a[1], a[2])));
            Ret {
                eax: a[2],
                ..Ret::default()
            }
        });
        let (writer, stream) = stream_owner(&mut e, 0);
        let data = source_data(&mut e);
        e.mem.set_u32(data + 0xe0, 0x7000);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, data);
        e.call(0x00cc6ef0, &args![this, writer]);
        let sequence: Vec<(u32, usize)> = written
            .borrow()
            .iter()
            .map(|(at, bytes)| (at - data, bytes.len()))
            .collect();
        assert_eq!(
            sequence,
            vec![
                (0x20, 0x40),
                (0x60, 0x40),
                (0x11c, 4),
                (0x12c, 4),
                (0x130, 4),
                (0x104, 4),
                (0x108, 4),
                (0xf8, 4),
                (0xa2, 1)
            ]
        );
        assert_eq!(written.borrow()[8].1, vec![0x5a]);
        assert_eq!(e.mem.u32(stream + 4), 0x80 + 24 + 1);
        assert_eq!(calls_to(&log, CINFO_SAVE_BINARY), vec![vec![this, writer]]);
        assert_eq!(calls_to(&log, CINFO_SAVE_MOTOR), vec![vec![0x7000, writer]]);
        // Without a motor the flag byte is not written.
        written.borrow_mut().clear();
        e.mem.set_u32(data + 0xe0, 0);
        e.call(0x00cc6ef0, &args![this, writer]);
        assert_eq!(written.borrow().len(), 8);
        assert_eq!(calls_to(&log, CINFO_SAVE_MOTOR)[1], vec![0, writer]);
    }

    /// A viewer-string array object with room for `capacity` entries, and doubles for the grow
    /// and set-at methods that keep the 16-bit size and capacity.
    fn viewer_array(e: &mut Engine, log: &Log, capacity: u16) -> u32 {
        let array = e.mem.alloc(0x10);
        let table = e.mem.alloc(0x100);
        e.mem.set_u32(array + 4, table);
        e.mem.set_u16(array + 8, capacity);
        e.mem.set_u16(array + 0xe, 4);
        let kept = Rc::clone(log);
        e.register_double(STRINGS_GROW, move |e, a| {
            kept.borrow_mut().push((STRINGS_GROW, a.to_vec()));
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        e.register_double(STRINGS_SET_AT, move |e, a| {
            let table = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(table + 4 * a[1], value);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        array
    }

    #[test]
    fn viewer_strings_append_vectors_angles_and_torque() {
        let (mut e, log) = engine(&[CINFO_GET_VIEWER_STRINGS, COPY_POINT]);
        let next = Rc::new(RefCell::new(0x100u32));
        for address in [NI_POINT3_GET_VIEWER_STRING, NI_GET_VIEWER_STRING_FLOAT] {
            let kept = Rc::clone(&log);
            let next = Rc::clone(&next);
            e.register_double(address, move |_, a| {
                kept.borrow_mut().push((address, a.to_vec()));
                *next.borrow_mut() += 1;
                Ret {
                    eax: *next.borrow() - 1,
                    ..Ret::default()
                }
            });
        }
        e.set_global(DEGREES_PER_RADIAN, 57.295_776_f64);
        let array = viewer_array(&mut e, &log, 4);
        let data = e.mem.alloc(0x140);
        for (index, offset) in [0x11cu32, 0x12c, 0x130, 0x104, 0x108].iter().enumerate() {
            e.mem.set_f32(data + offset, 0.5 + index as f32);
        }
        e.mem.set_u32(data + 0xf8, 0x7f80_0001);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, data);
        e.call(0x00cc7090, &args![this, array]);
        let points = calls_to(&log, COPY_POINT);
        let offsets: Vec<u32> = points.iter().map(|c| c[1] - data).collect();
        assert_eq!(offsets, vec![0x50, 0x30, 0x20, 0x90, 0x70, 0x60]);
        let names: Vec<u32> = calls_to(&log, NI_POINT3_GET_VIEWER_STRING)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(
            names,
            vec![
                0x010c_839c,
                0x010c_9c74,
                0x010c_9c68,
                0x010c_8390,
                0x010c_9c5c,
                0x010c_9c50
            ]
        );
        let floats = calls_to(&log, NI_GET_VIEWER_STRING_FLOAT);
        assert_eq!(floats.len(), 6);
        assert_eq!(floats[0][0], 0x010c_9c40);
        assert_eq!(floats[0][1], ((0.5f64 * 57.295_776_f64) as f32).to_bits());
        assert_eq!(floats[4][0], 0x010c_9c00);
        assert_eq!(floats[4][1], ((4.5f64 * 57.295_776_f64) as f32).to_bits());
        assert_eq!(floats[5], vec![0x010c_8340, 0x7fc0_0001]);
        // Twelve strings were appended; the array grew when it filled (capacity 4, step 4).
        assert_eq!(e.mem.u16(array + 0xa), 12);
        let table = e.mem.u32(array + 4);
        assert_eq!(e.mem.u32(table), 0x100);
        assert_eq!(e.mem.u32(table + 4 * 11), 0x10b);
        let grows = calls_to(&log, STRINGS_GROW);
        assert_eq!(grows[0], vec![array, 8]);
        assert_eq!(
            calls_to(&log, CINFO_GET_VIEWER_STRINGS),
            vec![vec![this, array]]
        );
    }

    #[test]
    fn viewer_strings_without_data_only_run_the_base() {
        let (mut e, log) = engine(&[CINFO_GET_VIEWER_STRINGS, COPY_POINT]);
        let this = e.mem.alloc(0x10);
        e.call(0x00cc7090, &args![this, 0x8000u32]);
        assert_eq!(order(&log), vec![CINFO_GET_VIEWER_STRINGS]);
    }
}
