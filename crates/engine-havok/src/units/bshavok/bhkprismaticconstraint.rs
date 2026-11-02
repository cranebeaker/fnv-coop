//! `BSHavok/bhkprismaticconstraint.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkPrismaticConstraint` (0x10 bytes) wraps a Havok constraint: `+8` holds the Havok object and
//! `+0xC` the construction info (`pInfo`). The construction info is a 0x14-byte block (vtable
//! `010c9d60`, called `hkPrismaticConstraintCinfo` in the Ghidra database) whose address
//! `pInfo` holds plus 4: the functions that take the block itself get `pInfo - 4`. Its word at
//! `+4` points at the Havok constraint data, a 0xD0-byte block with two 0x40-byte matrices at
//! `+0x20` and `+0x60`, floats at `+0xB4`, `+0xC8` and `+0xCC`, a motor at `+0xAC` and a byte
//! at `+0xA2`.

#[allow(unused_imports)]
use crate::prelude::*;

use super::bhknicollisionobject::enter_context;
use super::bhknitristripsshape::havok_allocator;
use super::bhkorienthingedbodyaction::copy_aligned_vector;

/// The class's own vtable (`010c9c84`) and the vtable of the base class its inlined constructor
/// stores on the way (`0108febc`, `bhkConstraint`'s).
const VTABLE: u32 = 0x010c_9c84;
const BASE_VTABLE: u32 = 0x0108_febc;
/// Static object counts: the base class's (`01268188`) and the class's own (`01268618`, which the
/// destructor decrements).
const BASE_COUNT: u32 = 0x0126_8188;
const OBJECT_COUNT: u32 = 0x0126_8618;
/// The vtable of the construction info block.
const CINFO_VTABLE: u32 = 0x010c_9d60;

/// Helpers of the game and Havok heaps.
const OPERATOR_NEW: u32 = 0x00aa_13e0;
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;

/// Base class methods: the `bhkRefObject` constructor (`00c87c30`), the `bhkSerializable` kill
/// (`00c876a0`), the destructor body of the base (`00c89640`), the clone lock and unlock
/// (`00c876f0`, `00c87700`), `bhkConstraint::Init` (`00c89dd0`), `CopyMembers`-like step of the
/// clone (`00c89730`, `(this, clone, process)`), the setter of the construction info for the
/// Havok object (`00c89c80`), and the shared constraint-info helpers: `GetViewerStrings`
/// (`00c8a560`), the data cloning tail (`00c8a1b0`, `(this, data, process)`), the stream
/// readers and writers (`00c8a230` / `00c8a280`, `(this, stream)`), the motor reader
/// (`00c8a330`, `(this, stream)`) and writer (`00c8a4b0`, `stdcall` `(motor, stream)`).
const REF_OBJECT_CONSTRUCTOR: u32 = 0x00c8_7c30;
const SERIALIZABLE_KILL: u32 = 0x00c8_76a0;
const BASE_DESTRUCTOR_BODY: u32 = 0x00c8_9640;
const CLONE_LOCK: u32 = 0x00c8_76f0;
const CLONE_UNLOCK: u32 = 0x00c8_7700;
const BASE_INIT: u32 = 0x00c8_9dd0;
const BASE_CLONE_STEP: u32 = 0x00c8_9730;
const BASE_SET_CINFO: u32 = 0x00c8_9c80;
const CINFO_GET_VIEWER_STRINGS: u32 = 0x00c8_a560;
const CINFO_CLONE_DATA: u32 = 0x00c8_a1b0;
const CINFO_LOAD_BINARY: u32 = 0x00c8_a230;
const CINFO_SAVE_BINARY: u32 = 0x00c8_a280;
const CINFO_READ_MOTOR: u32 = 0x00c8_a330;
const CINFO_WRITE_MOTOR: u32 = 0x00c8_a4b0;
/// The Havok data constructor (`00d4fe80`, `(block)`, answers the object), its motor setter
/// (`00d4f850`, `(data, motor)`), the release of a reference (`00c905b0`) and the byte swap of
/// twelve words (`00c8cdb0`, cdecl `(address)`).
const DATA_CONSTRUCT: u32 = 0x00d4_fe80;
const DATA_SET_MOTOR: u32 = 0x00d4_f850;
const HAVOK_RELEASE: u32 = 0x00c9_05b0;
const SWAP_TWELVE_WORDS: u32 = 0x00c8_cdb0;

layout! {
    /// `bhkPrismaticConstraint` (`bhkConstraint` in the Xbox PDB), 0x10 bytes.
    pub struct BhkPrismaticConstraint: 0x10 {
        /// `phkObject` (Xbox PDB, `bhkRefObject`): the Havok constraint.
        0x08 phkObject: Ptr,
        /// `pInfo` (Xbox PDB, `bhkSerializable`): the construction info plus 4.
        0x0c pInfo: Ptr,
    }
}

layout! {
    /// The construction info block (0x14 bytes) as these functions use it.
    pub struct PrismaticCinfo: 0x14 {
        /// The Havok constraint data.
        0x04 data: Ptr,
    }
}

/// Stores what the construction info constructors store: the vtable, the data pointer cleared,
/// `+8` set to 1 and `+0xC`, `+0x10` cleared.
fn construct_cinfo(e: &mut Engine, block: u32) {
    e.mem.set_u32(block + 4, 0);
    e.mem.set_u32(block + 8, 1);
    e.mem.set_u32(block + 0xc, 0);
    e.mem.set_u32(block + 0x10, 0);
    e.mem.set_u32(block, CINFO_VTABLE);
}

/// What the inlined constructors of the two base classes do after the allocation: the base
/// constructor, the vtables and object counts, and the cleared `pInfo` (the class's own vtable
/// and count come last).
fn construct(e: &mut Engine, this: u32) {
    e.call(REF_OBJECT_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this, BASE_VTABLE);
    e.mem.set_u32(this + 0xc, 0);
    let count = e.mem.u32(BASE_COUNT);
    e.mem.set_u32(BASE_COUNT, count.wrapping_add(1));
    e.mem.set_u32(this, VTABLE);
    let count = e.mem.u32(OBJECT_COUNT);
    e.mem.set_u32(OBJECT_COUNT, count.wrapping_add(1));
}

/// One stream transfer of `size` bytes at `address`: the stream's input object (the pointer at
/// `stream + 0x24c`, read function in slot `+8`) when `input`, its output object (`stream +
/// 0x250`, write function in slot `+0xC`) otherwise. The function is called cdecl as `(io,
/// address, size, &size, 1)` with the size in a local, or, for a `raw` transfer, as `(io, address,
/// size, 0, 0)`; the byte count it answers is added to the counter at `io + 4`.
fn transfer(e: &mut Engine, stream: u32, input: bool, address: u32, size: u32, raw: bool) {
    let (io_pointer, slot) = if input { (0x24c, 8) } else { (0x250, 0xc) };
    let io = e.mem.u32(stream + io_pointer);
    let function = e.mem.u32(io + slot);
    let transferred = if raw {
        e.call(function, &args![io, address, size, 0u32, 0u32])
            .u32()
    } else {
        e.with_stack(4, |e, cell| {
            e.mem.set_u32(cell.addr(), size);
            e.call(function, &args![io, address, size, cell, 1u32])
                .u32()
        })
    };
    let total = e.mem.u32(io + 4);
    e.mem.set_u32(io + 4, total.wrapping_add(transferred));
}

/// Byte-swaps the four words at `address`.
fn swap_four_words(e: &mut Engine, address: u32) {
    for word in 0..4 {
        let value = e.mem.u32(address + 4 * word);
        e.mem.set_u32(address + 4 * word, value.swap_bytes());
    }
}

// Translated from 00cc5080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00cc5080` (a 5-byte jump to `00c8a560`; the Ghidra database labels this address
/// `hkBallAndSocketConstraintCinfo::GetViewerStrings`, the code being shared between classes):
/// `this` and the stack word are passed on.
pub fn fn_00cc5080(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(CINFO_GET_VIEWER_STRINGS, &args![this, strings]);
}

// Translated from 00cc7450 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc7450 - C++ exception unwinding (SEH frame state) is not translated.
/// `bhkPrismaticConstraint::~bhkPrismaticConstraint` (engine map): the destructor body. Sets the
/// vtable, kills the serializable base (`00c876a0`), decrements the class's object count and
/// runs the base destructor body (`00c89640`).
pub fn bhk_prismatic_constraint_destructor(e: &mut Engine, this: Ptr<BhkPrismaticConstraint>) {
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.mem.u32(OBJECT_COUNT);
    e.mem.set_u32(OBJECT_COUNT, count.wrapping_sub(1));
    e.call(BASE_DESTRUCTOR_BODY, &args![this]);
}

// Translated from 00cc7500 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc7500 - C++ exception unwinding (SEH frame state) is not translated.
/// `fn_00cc7500` (a constructing allocator for the class): takes 0x10 bytes from `operator new`
/// (`00aa13e0`) and, when that succeeded, constructs a `bhkPrismaticConstraint` in them (the base
/// constructor `00c87c30`, the base vtable and count, `pInfo` cleared, the class's own vtable and
/// count). Returns the object, or null.
pub fn fn_00cc7500(e: &mut Engine) -> Ptr<BhkPrismaticConstraint> {
    let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    construct(e, block);
    Ptr::new(block)
}

// Translated from 00cc7590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00cc7590` (constructor of the construction info block): clears the data pointer (`+4`),
/// sets `+8` to 1, clears `+0xC` and `+0x10` and stores the vtable `010c9d60`. Returns `this`.
pub fn fn_00cc7590(e: &mut Engine, this: Ptr<PrismaticCinfo>) -> Ptr<PrismaticCinfo> {
    construct_cinfo(e, this.addr());
    this
}

// Translated from 00cc75b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkPrismaticConstraint::Init` (engine map): runs `bhkConstraint::Init` (`00c89dd0`) with
/// `argument`. Then takes the Havok constraint (the pointer at `+8` of `this` when there is one,
/// whose word at `+0xC` is its data pointer; otherwise the pointer the word at `pInfo` points
/// at, if `pInfo` is not null). When that object's float at `+0xB4` differs from 0.0 (a NaN does)
/// and `+8` holds an object, sets the byte at `+0x1D` of that object to 1.
pub fn bhk_prismatic_constraint_init(
    e: &mut Engine,
    this: Ptr<BhkPrismaticConstraint>,
    argument: Ptr,
) {
    e.call(BASE_INIT, &args![this, argument]);
    let havok = if this.is_null() {
        0
    } else {
        e.get(this, BhkPrismaticConstraint::phkObject).addr()
    };
    let data = if havok != 0 {
        e.mem.u32(havok + 0xc)
    } else {
        let info = e.mem.u32(this.addr() + 0xc);
        if info == 0 {
            return;
        }
        e.mem.u32(info)
    };
    if data == 0 {
        return;
    }
    // FLDZ; FCOMP: skipped only when the float equals 0.0.
    let limit = e.mem.f32(data + 0xb4);
    if limit != 0.0 {
        let havok = e.get(this, BhkPrismaticConstraint::phkObject).addr();
        if havok != 0 {
            e.mem.set_u8(havok + 0x1d, 1);
        }
    }
}

// Translated from 00cc7600 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc7600 - C++ exception unwinding (SEH frame state) is not translated.
/// `bhkPrismaticConstraint`'s construction info `CloneData` (engine map: `CloneData`; `this` is
/// the construction info block, `data` the target Havok data or null, `process` the cloning
/// process). With the thread block's `+0x2b4` word at 0x10: the source is the data at `this + 4`.
/// A null `data` is made from 0xD0 bytes of the Havok allocator (slot `+4`, with 0xD0 stored in
/// the 16-bit word at `+4` of the block) and the constructor `00d4fe80`. Copies the eight
/// 16-byte vectors at `+0x20..+0xA0` (aligned copies) and the floats at `+0xCC`, `+0xC8` and
/// `+0xB4` to it (the floats through the x87, quieting signalling NaNs). When the float at
/// `process + 0x10` (the scale, via the stack) is not exactly 1.0, multiplies the vectors at
/// `+0x50` and `+0x90` and the floats at `+0xC8` and `+0xCC` by it. When the source has a motor
/// (`+0xAC`), gets its Havok object (virtual slot `+0xC`), gives it to the target
/// (`00d4f850`), releases it (`00c905b0`) and copies the byte at `+0xA2`. Finally runs
/// `00c8a1b0(this, data, process)`.
pub fn bhk_prismatic_constraint_clone_data(
    e: &mut Engine,
    this: Ptr<PrismaticCinfo>,
    data: Ptr,
    process: Ptr,
) {
    let (word, saved) = enter_context(e);
    let source = e.get(this, PrismaticCinfo::data).addr();
    let mut target = data.addr();
    if target == 0 {
        let allocator = havok_allocator(e);
        let block = e.vcall(allocator, 4, &args![0xd0u32]).u32();
        e.mem.set_u16(block + 4, 0xd0);
        target = e.call(DATA_CONSTRUCT, &args![block]).u32();
    }
    for offset in (0x20..0xa0).step_by(0x10) {
        copy_aligned_vector(e, source + offset, target + offset);
    }
    for offset in [0xccu32, 0xc8, 0xb4] {
        let value = x87_quiet_bits(e.mem.u32(source + offset));
        e.mem.set_u32(target + offset, value);
    }
    let scale = x87_quiet(e.mem.f32(process.addr() + 0x10));
    if scale != 1.0 {
        for offset in [0x50u32, 0x90] {
            for lane in 0..4 {
                let address = target + offset + 4 * lane;
                let value = e.mem.f32(address);
                e.mem.set_f32(address, scale * value);
            }
        }
        for offset in [0xc8u32, 0xcc] {
            let value = e.mem.f32(target + offset);
            e.mem.set_f32(target + offset, scale * value);
        }
    }
    let motor_holder = e.mem.u32(source + 0xac);
    if motor_holder != 0 {
        let motor = e.vcall(motor_holder, 0xc, &args![]).u32();
        e.call(DATA_SET_MOTOR, &args![target, motor]);
        e.call(HAVOK_RELEASE, &args![motor]);
        let byte = e.mem.u8(source + 0xa2);
        e.mem.set_u8(target + 0xa2, byte);
    }
    e.call(CINFO_CLONE_DATA, &args![this, target, process]);
    e.mem.set_u32(word, saved);
}

// Translated from 00cc77e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc77e0 - C++ exception unwinding (SEH frame state) is not translated.
/// `bhkPrismaticConstraint`'s construction info `CreateData` (engine map): with the thread block's
/// `+0x2b4` word at 0x10, when the data pointer at `this + 4` is null makes the Havok data as
/// `CloneData` does for a null target (0xD0 bytes from the Havok allocator, 0xD0 stored in the
/// 16-bit word at `+4`, constructor `00d4fe80`) and stores it there. Returns 0xD0.
pub fn bhk_prismatic_constraint_create_data(e: &mut Engine, this: Ptr<PrismaticCinfo>) -> u32 {
    let (word, saved) = enter_context(e);
    if e.get(this, PrismaticCinfo::data).is_null() {
        let allocator = havok_allocator(e);
        let block = e.vcall(allocator, 4, &args![0xd0u32]).u32();
        e.mem.set_u16(block + 4, 0xd0);
        let data = e.call(DATA_CONSTRUCT, &args![block]).u32();
        e.set(this, PrismaticCinfo::data, Ptr::new(data));
    }
    e.mem.set_u32(word, saved);
    0xd0
}

// Translated from 00cc7890 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc7890 - C++ exception unwinding (SEH frame state) is not translated.
/// `bhkPrismaticConstraint::CreateClone` (engine map): with the thread block's `+0x2b4` word at
/// 0x10 and the clone lock held (`00c876f0` / `00c87700`), allocates and constructs a new
/// constraint as `00cc7500` does, then runs `00c89730(this, clone, process)` (the clone's
/// `CopyMembers`; the clone is null when the allocation failed). Returns the new constraint.
pub fn bhk_prismatic_constraint_create_clone(
    e: &mut Engine,
    this: Ptr<BhkPrismaticConstraint>,
    process: Ptr,
) -> Ptr<BhkPrismaticConstraint> {
    let (word, saved) = enter_context(e);
    e.call(CLONE_LOCK, &args![]);
    let clone = fn_00cc7500(e);
    e.call(BASE_CLONE_STEP, &args![this, clone, process]);
    e.call(CLONE_UNLOCK, &args![]);
    e.mem.set_u32(word, saved);
    clone
}

// Translated from 00cc7970 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc7970 - C++ exception unwinding (SEH frame state) is not translated.
/// `bhkPrismaticConstraint::CreateCinfo` (engine map): with the thread block's `+0x2b4` word at
/// 0x10, sets the byte at `created` to whether `pInfo` was empty. If it was, takes 0x14 bytes from
/// the memory manager (`00aa3e40`, no alignment) and, when that worked, makes them the
/// construction info block (as `fn_00cc7590`); `pInfo` becomes the block plus 4 (null when the
/// allocation failed). When there is a Havok constraint (`+8`), runs `00c89c80(this, block)` with
/// the block (`pInfo - 4`, or null). Returns `pInfo`.
pub fn bhk_prismatic_constraint_create_cinfo(
    e: &mut Engine,
    this: Ptr<BhkPrismaticConstraint>,
    created: Ptr,
) -> Ptr {
    let (word, saved) = enter_context(e);
    let empty = e.get(this, BhkPrismaticConstraint::pInfo).is_null();
    e.mem.set_u8(created.addr(), u8::from(empty));
    if empty {
        let block = e
            .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, 0x14u32])
            .u32();
        let info = if block == 0 {
            0
        } else {
            construct_cinfo(e, block);
            block + 4
        };
        e.set(this, BhkPrismaticConstraint::pInfo, Ptr::new(info));
        if !e.get(this, BhkPrismaticConstraint::phkObject).is_null() {
            let argument = if info == 0 { 0 } else { info - 4 };
            e.call(BASE_SET_CINFO, &args![this, argument]);
        }
    }
    let info = e.get(this, BhkPrismaticConstraint::pInfo);
    e.mem.set_u32(word, saved);
    info
}

// Translated from 00cc7a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkPrismaticConstraint`'s construction info `LoadBinary` (engine map; `this` is the
/// construction info block). Runs `00c8a230(this, stream)`, then reads from the stream's input
/// object (slot `+8`): 0x40 bytes raw to `data + 0x20` and to `data + 0x60`, then the four-byte
/// floats at `+0xC8`, `+0xCC` and `+0xB4` (the data at `this + 4`). When the stream's byte at
/// `+0x1E5` is not 1, byte-swaps the words at `+0x50..+0x60` and `+0x90..+0xA0` and swaps the
/// twelve words at `+0x20` and `+0x60` through `00c8cdb0`. When the stream version (`stream +
/// 4`) is above 0x1D, reads a motor (`00c8a330(this, stream)`), gives it to the data
/// (`00d4f850`) and, when there is one, reads the byte at `+0xA2` and releases the motor
/// (`00c905b0`).
pub fn bhk_prismatic_constraint_load_binary(
    e: &mut Engine,
    this: Ptr<PrismaticCinfo>,
    stream: Ptr,
) {
    let stream_addr = stream.addr();
    e.call(CINFO_LOAD_BINARY, &args![this, stream]);
    let data = e.get(this, PrismaticCinfo::data).addr();
    transfer(e, stream_addr, true, data + 0x20, 0x40, true);
    transfer(e, stream_addr, true, data + 0x60, 0x40, true);
    transfer(e, stream_addr, true, data + 0xc8, 4, false);
    transfer(e, stream_addr, true, data + 0xcc, 4, false);
    transfer(e, stream_addr, true, data + 0xb4, 4, false);
    if e.mem.u8(stream_addr + 0x1e5) != 1 {
        swap_four_words(e, data + 0x50);
        e.call(SWAP_TWELVE_WORDS, &args![data + 0x20]);
        swap_four_words(e, data + 0x90);
        e.call(SWAP_TWELVE_WORDS, &args![data + 0x60]);
    }
    if e.mem.u32(stream_addr + 4) > 0x1d {
        let motor = e.call(CINFO_READ_MOTOR, &args![this, stream]).u32();
        e.call(DATA_SET_MOTOR, &args![data, motor]);
        if motor != 0 {
            transfer(e, stream_addr, true, data + 0xa2, 1, false);
            e.call(HAVOK_RELEASE, &args![motor]);
        }
    }
}

// Translated from 00cc7bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkPrismaticConstraint`'s construction info `SaveBinary` (engine map; `this` is the
/// construction info block). Runs `00c8a280(this, stream)`, then writes to the stream's output
/// object (slot `+0xC`), each as `(io, address, size, &size, 1)`: 0x40 bytes from `data + 0x20`
/// and `data + 0x60`, then the four-byte floats at `+0xC8`, `+0xCC` and `+0xB4` (the data at
/// `this + 4`). Runs `00c8a4b0(this, motor, stream)` with the motor at `+0xAC`; when there is one
/// writes the byte at `+0xA2`.
pub fn bhk_prismatic_constraint_save_binary(
    e: &mut Engine,
    this: Ptr<PrismaticCinfo>,
    stream: Ptr,
) {
    let stream_addr = stream.addr();
    e.call(CINFO_SAVE_BINARY, &args![this, stream]);
    let data = e.get(this, PrismaticCinfo::data).addr();
    transfer(e, stream_addr, false, data + 0x20, 0x40, false);
    transfer(e, stream_addr, false, data + 0x60, 0x40, false);
    transfer(e, stream_addr, false, data + 0xc8, 4, false);
    transfer(e, stream_addr, false, data + 0xcc, 4, false);
    transfer(e, stream_addr, false, data + 0xb4, 4, false);
    let motor = e.mem.u32(data + 0xac);
    // 00cc7ca0: `00c8a4b0` is stdcall (RET 8) and does not read the ECX (this) the
    // caller sets.
    e.call(CINFO_WRITE_MOTOR, &args![motor, stream]);
    if e.mem.u32(data + 0xac) != 0 {
        transfer(e, stream_addr, false, data + 0xa2, 1, false);
    }
}

// Translated from 00cc74e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkPrismaticConstraint::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00cc7450`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x10 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bhk_prismatic_constraint_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00cc_7450;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x10u32]);
    }
    this
}

// Translated from 00cc74c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkPrismaticConstraint::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_861c.
pub fn bhk_prismatic_constraint_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_861c
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cc5080, fn_00cc5080(Ptr, Ptr)),
        entry!(
            0x00cc7450,
            bhk_prismatic_constraint_destructor(Ptr<BhkPrismaticConstraint>)
        ),
        entry!(0x00cc74c0, bhk_prismatic_constraint_get_r_t_t_i() -> u32),
        entry!(0x00cc74e0, bhk_prismatic_constraint_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00cc7500, fn_00cc7500() -> Ptr<BhkPrismaticConstraint>),
        entry!(
            0x00cc7590,
            fn_00cc7590(Ptr<PrismaticCinfo>) -> Ptr<PrismaticCinfo>
        ),
        entry!(
            0x00cc75b0,
            bhk_prismatic_constraint_init(Ptr<BhkPrismaticConstraint>, Ptr)
        ),
        entry!(
            0x00cc7600,
            bhk_prismatic_constraint_clone_data(Ptr<PrismaticCinfo>, Ptr, Ptr)
        ),
        entry!(
            0x00cc77e0,
            bhk_prismatic_constraint_create_data(Ptr<PrismaticCinfo>) -> u32
        ),
        entry!(
            0x00cc7890,
            bhk_prismatic_constraint_create_clone(
                Ptr<BhkPrismaticConstraint>,
                Ptr,
            ) -> Ptr<BhkPrismaticConstraint>
        ),
        entry!(
            0x00cc7970,
            bhk_prismatic_constraint_create_cinfo(Ptr<BhkPrismaticConstraint>, Ptr) -> Ptr
        ),
        entry!(
            0x00cc7a40,
            bhk_prismatic_constraint_load_binary(Ptr<PrismaticCinfo>, Ptr)
        ),
        entry!(
            0x00cc7bc0,
            bhk_prismatic_constraint_save_binary(Ptr<PrismaticCinfo>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Words = Vec<u32>;

    const ALLOCATOR_ROUTER: u32 = 0x00c8_5750;
    const SIZED_DELETE_ADDRESS: u32 = 0x00aa_1460;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Words> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// An engine with this unit's globals mapped and the call log on.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(BASE_COUNT, 4);
        e.map(OBJECT_COUNT, 4);
        e.call_log = Some(vec![]);
        e
    }

    /// An object of `size` bytes whose vtable (a zeroed block) holds the `(slot offset, target)`
    /// pairs.
    fn object_with_slots(e: &mut Engine, size: u32, slots: &[(u32, u32)]) -> u32 {
        let table = e.mem.alloc(0x100);
        for (slot, target) in slots {
            e.mem.set_u32(table + slot, *target);
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, table);
        object
    }

    /// A 16-byte aligned block of `size` bytes.
    fn aligned_block(e: &mut Engine, size: u32) -> u32 {
        let raw = e.mem.alloc(size + 0x10);
        (raw + 0xf) & !0xf
    }

    #[test]
    fn scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = engine();
        e.register(0x00cc_7450, |_, _| Ret::default());
        e.register(SIZED_DELETE_ADDRESS, |_, _| Ret::default());
        let this = 0x5000u32;
        assert_eq!(e.call(0x00cc74e0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, SIZED_DELETE_ADDRESS).is_empty());
        assert_eq!(e.call(0x00cc74e0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, SIZED_DELETE_ADDRESS), vec![vec![this, 0x10]]);
        assert_eq!(calls_to(&e, 0x00cc_7450), vec![vec![this]; 2]);
    }

    #[test]
    fn get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cc_74c0, &args![]).u32(), 0x0126_861c);
    }

    #[test]
    fn the_thunk_forwards_both_words() {
        let mut e = engine();
        e.register(CINFO_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.call(0x00cc5080, &args![0x1000u32, 0x2000u32]);
        assert_eq!(
            calls_to(&e, CINFO_GET_VIEWER_STRINGS),
            vec![vec![0x1000, 0x2000]]
        );
    }

    #[test]
    fn destructor_kills_the_base_counts_down_and_runs_the_base_body() {
        let mut e = engine();
        e.register(SERIALIZABLE_KILL, |_, _| Ret::default());
        e.register(BASE_DESTRUCTOR_BODY, |_, _| Ret::default());
        e.mem.set_u32(OBJECT_COUNT, 5);
        let this = e.new_object::<BhkPrismaticConstraint>();
        e.call(0x00cc7450, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        assert_eq!(e.mem.u32(OBJECT_COUNT), 4);
        assert_eq!(calls_to(&e, SERIALIZABLE_KILL), vec![vec![this.addr()]]);
        assert_eq!(calls_to(&e, BASE_DESTRUCTOR_BODY), vec![vec![this.addr()]]);
    }

    #[test]
    fn fn_00cc7500_constructs_the_object_or_answers_null() {
        let mut e = engine();
        e.register(REF_OBJECT_CONSTRUCTOR, |_, _| Ret::default());
        let block = e.mem.alloc(0x10);
        e.mem.set_u32(block + 0xc, 0x99);
        e.mem.set_u32(BASE_COUNT, 1);
        e.mem.set_u32(OBJECT_COUNT, 3);
        e.register_double(OPERATOR_NEW, move |_, _| ret(block));
        assert_eq!(e.call(0x00cc7500, &args![]).u32(), block);
        assert_eq!(e.mem.u32(block), VTABLE);
        assert_eq!(e.mem.u32(block + 0xc), 0);
        assert_eq!(e.mem.u32(BASE_COUNT), 2);
        assert_eq!(e.mem.u32(OBJECT_COUNT), 4);
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x10]]);
        assert_eq!(calls_to(&e, REF_OBJECT_CONSTRUCTOR), vec![vec![block]]);

        e.register(OPERATOR_NEW, |_, _| ret(0));
        assert_eq!(e.call(0x00cc7500, &args![]).u32(), 0);
        assert_eq!(calls_to(&e, REF_OBJECT_CONSTRUCTOR).len(), 1);
    }

    #[test]
    fn fn_00cc7590_initialises_the_cinfo_block() {
        let mut e = engine();
        let block = e.mem.alloc(0x14);
        for offset in [4u32, 8, 0xc, 0x10] {
            e.mem.set_u32(block + offset, 0x77);
        }
        assert_eq!(e.call(0x00cc7590, &args![block]).u32(), block);
        assert_eq!(e.mem.u32(block), CINFO_VTABLE);
        assert_eq!(e.mem.u32(block + 4), 0);
        assert_eq!(e.mem.u32(block + 8), 1);
        assert_eq!(e.mem.u32(block + 0xc), 0);
        assert_eq!(e.mem.u32(block + 0x10), 0);
    }

    #[test]
    fn init_flags_the_havok_constraint_when_the_limit_is_not_zero() {
        // (limit bits, whether the byte is set)
        for (bits, flagged) in [
            (0u32, false),
            (0x8000_0000, false),
            (1.5f32.to_bits(), true),
            (0x7fc0_0000, true),
        ] {
            let mut e = engine();
            e.register(BASE_INIT, |_, _| Ret::default());
            let this = e.new_object::<BhkPrismaticConstraint>();
            let data = e.mem.alloc(0xd0);
            e.mem.set_u32(data + 0xb4, bits);
            let havok = e.mem.alloc(0x40);
            e.mem.set_u32(havok + 0xc, data);
            e.set(this, BhkPrismaticConstraint::phkObject, Ptr::new(havok));
            e.call(0x00cc75b0, &args![this, 0x5000u32]);
            assert_eq!(e.mem.u8(havok + 0x1d), u8::from(flagged), "bits {bits:#x}");
            assert_eq!(calls_to(&e, BASE_INIT), vec![vec![this.addr(), 0x5000]]);
        }
    }

    #[test]
    fn init_without_a_havok_constraint_follows_the_cinfo_pointer() {
        let mut e = engine();
        e.register(BASE_INIT, |_, _| Ret::default());
        let this = e.new_object::<BhkPrismaticConstraint>();
        // No constraint and no cinfo: nothing happens.
        e.call(0x00cc75b0, &args![this, 0u32]);
        // pInfo points at a word that is a null pointer: also nothing.
        let info = e.mem.alloc(8);
        e.set(this, BhkPrismaticConstraint::pInfo, Ptr::new(info));
        e.call(0x00cc75b0, &args![this, 0u32]);
        // pInfo's word points at data with a limit, but there is no Havok constraint to flag.
        let data = e.mem.alloc(0xd0);
        e.mem.set_f32(data + 0xb4, 2.0);
        e.mem.set_u32(info, data);
        e.call(0x00cc75b0, &args![this, 0u32]);
        assert_eq!(calls_to(&e, BASE_INIT).len(), 3);
    }

    /// A fixture for `CloneData`: a source block (`this`) whose data at `+4` has recognisable
    /// content, and the doubles it calls.
    fn clone_data_fixture(e: &mut Engine) -> (Ptr<PrismaticCinfo>, u32) {
        e.register(CINFO_CLONE_DATA, |_, _| Ret::default());
        e.register(DATA_SET_MOTOR, |_, _| Ret::default());
        e.register(HAVOK_RELEASE, |_, _| Ret::default());
        let this = e.new_object::<PrismaticCinfo>();
        let source = aligned_block(e, 0xd0);
        for word in 0..0x34 {
            e.mem.set_u32(source + 4 * word, 0x1000 + word);
        }
        e.set(this, PrismaticCinfo::data, Ptr::new(source));
        (this, source)
    }

    #[test]
    fn clone_data_copies_the_data_into_the_target_and_scales_it() {
        let mut e = engine();
        let (this, source) = clone_data_fixture(&mut e);
        let target = aligned_block(&mut e, 0xd0);
        // The vectors to scale are floats.
        for lane in 0..4 {
            e.mem.set_f32(source + 0x50 + 4 * lane, 1.0 + lane as f32);
            e.mem.set_f32(source + 0x90 + 4 * lane, 10.0 + lane as f32);
        }
        e.mem.set_f32(source + 0xc8, 3.0);
        e.mem.set_f32(source + 0xcc, 4.0);
        e.mem.set_u32(source + 0xb4, 0x7f80_0001);
        e.mem.set_u32(source + 0xac, 0);
        let process = e.mem.alloc(0x20);
        e.mem.set_f32(process + 0x10, 2.0);
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x42);

        e.call(0x00cc7600, &args![this, target, process]);

        // Unscaled words are copied as they are.
        assert_eq!(e.mem.u32(target + 0x20), 0x1000 + 8);
        assert_eq!(e.mem.u32(target + 0x6c), 0x1000 + 0x1b);
        // The vectors at +0x50 and +0x90 and the floats at +0xc8 and +0xcc are scaled.
        for lane in 0..4 {
            assert_eq!(
                e.mem.f32(target + 0x50 + 4 * lane),
                2.0 * (1.0 + lane as f32)
            );
            assert_eq!(
                e.mem.f32(target + 0x90 + 4 * lane),
                2.0 * (10.0 + lane as f32)
            );
        }
        assert_eq!(e.mem.f32(target + 0xc8), 6.0);
        assert_eq!(e.mem.f32(target + 0xcc), 8.0);
        // +0xb4 is copied through the x87, quieting the signalling NaN.
        assert_eq!(e.mem.u32(target + 0xb4), 0x7fc0_0001);
        // The words outside the copied ranges are untouched.
        assert_eq!(e.mem.u32(target + 0x1c), 0);
        assert_eq!(e.mem.u32(target + 0xa0), 0);
        assert_eq!(
            calls_to(&e, CINFO_CLONE_DATA),
            vec![vec![this.addr(), target, process]]
        );
        assert!(calls_to(&e, DATA_SET_MOTOR).is_empty());
        assert_eq!(e.mem.u32(word), 0x42);
    }

    #[test]
    fn clone_data_leaves_a_scale_of_exactly_one_alone_and_scales_for_a_nan() {
        for (bits, scaled) in [(1.0f32.to_bits(), false), (0x7fc0_0000, true)] {
            let mut e = engine();
            let (this, source) = clone_data_fixture(&mut e);
            let target = aligned_block(&mut e, 0xd0);
            e.mem.set_f32(source + 0xc8, 3.0);
            e.mem.set_u32(source + 0xac, 0);
            let process = e.mem.alloc(0x20);
            e.mem.set_u32(process + 0x10, bits);
            e.call(0x00cc7600, &args![this, target, process]);
            assert_eq!(e.mem.f32(target + 0xc8).is_nan(), scaled);
            if !scaled {
                assert_eq!(e.mem.f32(target + 0xc8), 3.0);
            }
        }
    }

    #[test]
    fn clone_data_makes_the_target_when_it_is_null_and_copies_the_motor() {
        let mut e = engine();
        let (this, source) = clone_data_fixture(&mut e);
        // The Havok allocator builds the block; the constructor answers an aligned object.
        let allocator = object_with_slots(&mut e, 0x10, &[(4, 0x7000_0004)]);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(ALLOCATOR_ROUTER, move |_, _| ret(router));
        let block = e.mem.alloc(0x20);
        e.register_double(0x7000_0004, move |_, _| ret(block));
        let made = aligned_block(&mut e, 0xd0);
        e.register_double(DATA_CONSTRUCT, move |_, _| ret(made));
        // The source has a motor: its holder answers the motor in slot +0xc.
        let holder = object_with_slots(&mut e, 0x10, &[(0xc, 0x7000_000c)]);
        e.register(0x7000_000c, |_, _| ret(0x9999));
        e.mem.set_u32(source + 0xac, holder);
        e.mem.set_u8(source + 0xa2, 0x5a);
        let process = e.mem.alloc(0x20);
        e.mem.set_f32(process + 0x10, 1.0);

        e.call(0x00cc7600, &args![this, 0u32, process]);

        assert_eq!(calls_to(&e, 0x7000_0004), vec![vec![allocator, 0xd0]]);
        assert_eq!(e.mem.u16(block + 4), 0xd0);
        assert_eq!(calls_to(&e, DATA_CONSTRUCT), vec![vec![block]]);
        assert_eq!(e.mem.u32(made + 0x40), 0x1000 + 0x10);
        assert_eq!(calls_to(&e, DATA_SET_MOTOR), vec![vec![made, 0x9999]]);
        assert_eq!(calls_to(&e, HAVOK_RELEASE), vec![vec![0x9999]]);
        assert_eq!(e.mem.u8(made + 0xa2), 0x5a);
        assert_eq!(
            calls_to(&e, CINFO_CLONE_DATA),
            vec![vec![this.addr(), made, process]]
        );
    }

    #[test]
    #[should_panic(expected = "not 16-byte aligned")]
    fn clone_data_faults_on_a_misaligned_source() {
        let mut e = engine();
        let (this, source) = clone_data_fixture(&mut e);
        e.set(this, PrismaticCinfo::data, Ptr::new(source + 4));
        let target = aligned_block(&mut e, 0xd0);
        let process = e.mem.alloc(0x20);
        e.call(0x00cc7600, &args![this, target, process]);
    }

    #[test]
    fn create_data_makes_the_havok_data_once_and_answers_its_size() {
        let mut e = engine();
        let allocator = object_with_slots(&mut e, 0x10, &[(4, 0x7000_0004)]);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(ALLOCATOR_ROUTER, move |_, _| ret(router));
        let block = e.mem.alloc(0x20);
        e.register_double(0x7000_0004, move |_, _| ret(block));
        e.register_double(DATA_CONSTRUCT, move |_, a| ret(a[0] + 0x100));
        let this = e.new_object::<PrismaticCinfo>();
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x42);
        assert_eq!(e.call(0x00cc77e0, &args![this]).u32(), 0xd0);
        assert_eq!(e.get(this, PrismaticCinfo::data).addr(), block + 0x100);
        assert_eq!(e.mem.u16(block + 4), 0xd0);
        assert_eq!(e.mem.u32(word), 0x42);
        // With the data in place, nothing is made.
        assert_eq!(e.call(0x00cc77e0, &args![this]).u32(), 0xd0);
        assert_eq!(calls_to(&e, DATA_CONSTRUCT).len(), 1);
    }

    #[test]
    fn create_clone_runs_the_clone_step_under_the_lock() {
        let mut e = engine();
        e.register(CLONE_LOCK, |_, _| Ret::default());
        e.register(CLONE_UNLOCK, |_, _| Ret::default());
        e.register(REF_OBJECT_CONSTRUCTOR, |_, _| Ret::default());
        e.register(BASE_CLONE_STEP, |_, _| Ret::default());
        let block = e.mem.alloc(0x10);
        e.register_double(OPERATOR_NEW, move |_, _| ret(block));
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x42);
        let this = e.new_object::<BhkPrismaticConstraint>();
        let clone = e.call(0x00cc7890, &args![this, 0x7000u32]).u32();
        assert_eq!(clone, block);
        assert_eq!(e.mem.u32(block), VTABLE);
        assert_eq!(
            calls_to(&e, BASE_CLONE_STEP),
            vec![vec![this.addr(), block, 0x7000]]
        );
        assert_eq!(e.mem.u32(word), 0x42);
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|c| c.0)
            .filter(|a| [CLONE_LOCK, CLONE_UNLOCK, BASE_CLONE_STEP].contains(a))
            .collect();
        assert_eq!(order, [CLONE_LOCK, BASE_CLONE_STEP, CLONE_UNLOCK]);

        e.register(OPERATOR_NEW, |_, _| ret(0));
        assert_eq!(e.call(0x00cc7890, &args![this, 0x7000u32]).u32(), 0);
        assert_eq!(
            calls_to(&e, BASE_CLONE_STEP)[1],
            vec![this.addr(), 0, 0x7000]
        );
    }

    #[test]
    fn create_cinfo_builds_the_block_once_and_hands_it_to_the_havok_constraint() {
        let mut e = engine();
        e.register(BASE_SET_CINFO, |_, _| Ret::default());
        let block = e.mem.alloc(0x14);
        e.register_double(MEMORY_MANAGER_ALLOCATE, move |_, _| ret(block));
        let this = e.new_object::<BhkPrismaticConstraint>();
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x42);
        let created = e.mem.alloc(4);

        let info = e.call(0x00cc7970, &args![this, created]).u32();
        assert_eq!(info, block + 4);
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(e.mem.u32(block), CINFO_VTABLE);
        assert_eq!(e.mem.u32(block + 8), 1);
        assert_eq!(e.mem.u32(block + 4), 0);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x14]]
        );
        // No Havok constraint: nothing is handed over.
        assert!(calls_to(&e, BASE_SET_CINFO).is_empty());
        assert_eq!(e.mem.u32(word), 0x42);

        // Again: found, not made.
        assert_eq!(e.call(0x00cc7970, &args![this, created]).u32(), block + 4);
        assert_eq!(e.mem.u8(created), 0);
        assert_eq!(calls_to(&e, MEMORY_MANAGER_ALLOCATE).len(), 1);

        // With a Havok constraint, the block is handed over.
        e.set(this, BhkPrismaticConstraint::phkObject, Ptr::new(0x7000));
        e.set(this, BhkPrismaticConstraint::pInfo, Ptr::NULL);
        e.call(0x00cc7970, &args![this, created]);
        assert_eq!(calls_to(&e, BASE_SET_CINFO), vec![vec![this.addr(), block]]);
    }

    #[test]
    fn create_cinfo_with_a_failed_allocation_hands_over_null() {
        let mut e = engine();
        e.register(BASE_SET_CINFO, |_, _| Ret::default());
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| ret(0));
        let this = e.new_object::<BhkPrismaticConstraint>();
        e.set(this, BhkPrismaticConstraint::phkObject, Ptr::new(0x7000));
        let created = e.mem.alloc(4);
        assert_eq!(e.call(0x00cc7970, &args![this, created]).u32(), 0);
        assert_eq!(calls_to(&e, BASE_SET_CINFO), vec![vec![this.addr(), 0]]);
    }

    /// A stream whose input object (`+0x24c`) and output object (`+0x250`) are one block `io`
    /// with the byte counter at `+4` set to 10 and the read function at `+8`, write at `+0xC`.
    fn stream_fixture(e: &mut Engine, version: u32, read: u32, write: u32) -> (u32, u32) {
        let stream = e.mem.alloc(0x260);
        let io = e.mem.alloc(0x20);
        e.mem.set_u32(stream + 4, version);
        e.mem.set_u32(stream + 0x24c, io);
        e.mem.set_u32(stream + 0x250, io);
        e.mem.set_u32(io + 4, 10);
        e.mem.set_u32(io + 8, read);
        e.mem.set_u32(io + 0xc, write);
        (stream, io)
    }

    #[test]
    fn load_binary_reads_the_data_and_swaps_it_for_other_endians() {
        for (flag, swapped) in [(1u8, false), (0, true)] {
            let mut e = engine();
            e.register(CINFO_LOAD_BINARY, |_, _| Ret::default());
            e.register(SWAP_TWELVE_WORDS, |_, _| Ret::default());
            let data = e.mem.alloc(0xd0);
            for word in 0..0x34 {
                e.mem.set_u32(data + 4 * word, 0x0102_0000 + word);
            }
            let this = e.new_object::<PrismaticCinfo>();
            e.set(this, PrismaticCinfo::data, Ptr::new(data));
            // The read function answers 0x10 bytes for raw reads, 4 for the others, and records
            // the arguments.
            e.register_double(0x7000_0008, |_, a| ret(if a[4] == 0 { 0x10 } else { 4 }));
            let (stream, io) = stream_fixture(&mut e, 0x1d, 0x7000_0008, 0x7000_000c);
            e.mem.set_u8(stream + 0x1e5, flag);

            e.call(0x00cc7a40, &args![this, stream]);

            let reads = calls_to(&e, 0x7000_0008);
            let summary: Vec<(u32, u32, u32)> = reads.iter().map(|c| (c[1], c[2], c[4])).collect();
            assert_eq!(
                summary,
                [
                    (data + 0x20, 0x40, 0),
                    (data + 0x60, 0x40, 0),
                    (data + 0xc8, 4, 1),
                    (data + 0xcc, 4, 1),
                    (data + 0xb4, 4, 1)
                ]
            );
            // The raw reads pass a null size pointer; the others a pointer to a local holding 4.
            assert_eq!((reads[0][3], reads[1][3]), (0, 0));
            assert!(reads.iter().all(|c| c[0] == io));
            assert_eq!(e.mem.u32(io + 4), 10 + 0x10 + 0x10 + 4 + 4 + 4);
            assert_eq!(
                calls_to(&e, CINFO_LOAD_BINARY),
                vec![vec![this.addr(), stream]]
            );
            if swapped {
                assert_eq!(e.mem.u32(data + 0x50), (0x0102_0000u32 + 0x14).swap_bytes());
                assert_eq!(e.mem.u32(data + 0x5c), (0x0102_0000u32 + 0x17).swap_bytes());
                assert_eq!(e.mem.u32(data + 0x90), (0x0102_0000u32 + 0x24).swap_bytes());
                assert_eq!(e.mem.u32(data + 0x9c), (0x0102_0000u32 + 0x27).swap_bytes());
                assert_eq!(
                    calls_to(&e, SWAP_TWELVE_WORDS),
                    vec![vec![data + 0x20], vec![data + 0x60]]
                );
            } else {
                assert_eq!(e.mem.u32(data + 0x50), 0x0102_0014);
                assert!(calls_to(&e, SWAP_TWELVE_WORDS).is_empty());
            }
            // The words next to the swapped vectors stay.
            assert_eq!(e.mem.u32(data + 0x4c), 0x0102_0013);
            assert_eq!(e.mem.u32(data + 0x60), 0x0102_0018);
        }
    }

    #[test]
    fn load_binary_reads_the_motor_only_for_newer_streams() {
        for (version, reads_motor) in [(0x1d, false), (0x1e, true)] {
            let mut e = engine();
            e.register(CINFO_LOAD_BINARY, |_, _| Ret::default());
            e.register(DATA_SET_MOTOR, |_, _| Ret::default());
            e.register(HAVOK_RELEASE, |_, _| Ret::default());
            e.register(CINFO_READ_MOTOR, |_, _| ret(0x9999));
            let data = e.mem.alloc(0xd0);
            let this = e.new_object::<PrismaticCinfo>();
            e.set(this, PrismaticCinfo::data, Ptr::new(data));
            e.register(0x7000_0008, |_, _| ret(1));
            let (stream, _) = stream_fixture(&mut e, version, 0x7000_0008, 0x7000_000c);
            e.mem.set_u8(stream + 0x1e5, 1);
            e.call(0x00cc7a40, &args![this, stream]);
            assert_eq!(
                calls_to(&e, CINFO_READ_MOTOR).len(),
                usize::from(reads_motor)
            );
            assert_eq!(calls_to(&e, DATA_SET_MOTOR).len(), usize::from(reads_motor));
            assert_eq!(calls_to(&e, HAVOK_RELEASE).len(), usize::from(reads_motor));
            // Five reads, and a sixth (the byte at +0xa2) when there is a motor.
            let reads = calls_to(&e, 0x7000_0008);
            assert_eq!(reads.len(), 5 + usize::from(reads_motor));
            if reads_motor {
                assert_eq!(reads[5][1..3], [data + 0xa2, 1]);
                assert_eq!(calls_to(&e, DATA_SET_MOTOR), vec![vec![data, 0x9999]]);
                assert_eq!(calls_to(&e, HAVOK_RELEASE), vec![vec![0x9999]]);
            }
        }
    }

    #[test]
    fn load_binary_without_a_motor_skips_the_byte_and_the_release() {
        let mut e = engine();
        e.register(CINFO_LOAD_BINARY, |_, _| Ret::default());
        e.register(DATA_SET_MOTOR, |_, _| Ret::default());
        e.register(HAVOK_RELEASE, |_, _| Ret::default());
        e.register(CINFO_READ_MOTOR, |_, _| ret(0));
        let data = e.mem.alloc(0xd0);
        let this = e.new_object::<PrismaticCinfo>();
        e.set(this, PrismaticCinfo::data, Ptr::new(data));
        e.register(0x7000_0008, |_, _| ret(1));
        let (stream, _) = stream_fixture(&mut e, 0x20, 0x7000_0008, 0x7000_000c);
        e.mem.set_u8(stream + 0x1e5, 1);
        e.call(0x00cc7a40, &args![this, stream]);
        assert_eq!(calls_to(&e, DATA_SET_MOTOR), vec![vec![data, 0]]);
        assert_eq!(calls_to(&e, 0x7000_0008).len(), 5);
        assert!(calls_to(&e, HAVOK_RELEASE).is_empty());
    }

    #[test]
    fn save_binary_writes_the_data_and_the_motor_byte() {
        for motor in [0u32, 0x9999] {
            let mut e = engine();
            e.register(CINFO_SAVE_BINARY, |_, _| Ret::default());
            e.register(CINFO_WRITE_MOTOR, |_, _| Ret::default());
            let data = e.mem.alloc(0xd0);
            e.mem.set_u32(data + 0xac, motor);
            let this = e.new_object::<PrismaticCinfo>();
            e.set(this, PrismaticCinfo::data, Ptr::new(data));
            let sizes = Rc::new(RefCell::new(Vec::new()));
            let seen = sizes.clone();
            e.register_double(0x7000_000c, move |e, a| {
                seen.borrow_mut().push(e.mem.u32(a[3]));
                ret(a[2])
            });
            let (stream, io) = stream_fixture(&mut e, 0x20, 0x7000_0008, 0x7000_000c);

            e.call(0x00cc7bc0, &args![this, stream]);

            let writes = calls_to(&e, 0x7000_000c);
            let summary: Vec<(u32, u32, u32)> = writes.iter().map(|c| (c[1], c[2], c[4])).collect();
            let mut expected = vec![
                (data + 0x20, 0x40, 1),
                (data + 0x60, 0x40, 1),
                (data + 0xc8, 4, 1),
                (data + 0xcc, 4, 1),
                (data + 0xb4, 4, 1),
            ];
            let mut total = 10 + 0x40 + 0x40 + 12;
            if motor != 0 {
                expected.push((data + 0xa2, 1, 1));
                total += 1;
            }
            assert_eq!(summary, expected);
            // The size cell passed holds the size.
            let cells: Vec<u32> = writes.iter().map(|c| c[2]).collect();
            assert_eq!(*sizes.borrow(), cells);
            assert_eq!(e.mem.u32(io + 4), total);
            assert_eq!(
                calls_to(&e, CINFO_SAVE_BINARY),
                vec![vec![this.addr(), stream]]
            );
            assert_eq!(calls_to(&e, CINFO_WRITE_MOTOR), vec![vec![motor, stream]]);
            assert!(calls_to(&e, 0x7000_0008).is_empty());
        }
    }
}
