//! `BSHavok/bhkballsocketconstraintchain.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkBallSocketConstraintChain` (Xbox PDB) is a `bhkConstraintChain` (0x10 bytes: `+8` the
//! Havok constraint, `+0xC` the cinfo block). `CreateCinfo` builds the 0x2c-byte cinfo block; the
//! cinfo functions named `hkBallSocketChainCinfo` in the Xbox PDB work on it, keeping the
//! data block (a 0x34-byte `hkpBallSocketChainData`) at `+4` and, in the data block, an array of
//! 0x20-byte elements (two 16-byte vectors each) that `CopyCinfo` and `CloneData` move around.
//!
//! The functions marked `Translation incomplete` carry an `FS:[0]` exception-unwinding frame
//! that is not modeled.

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00cc8d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConstraintChain::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00d53220`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x10 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bhk_constraint_chain_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00d5_3220;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x10u32]);
    }
    this
}

// Translated from 00cc8da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBallSocketConstraintChain::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00cc8ce0`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x10 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bhk_ball_socket_constraint_chain_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00cc_8ce0;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x10u32]);
    }
    this
}

// Translated from 00cc8d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBallSocketConstraintChain::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_8640.
pub fn bhk_ball_socket_constraint_chain_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_8640
}

// Translated from 00cc8d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConstraintChain::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_8f84.
pub fn bhk_constraint_chain_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_8f84
}

/// Per-thread word (`TLS + 0x2b4`) that the allocating functions set to 0x10 around their work.
const SCOPE_WORD: u32 = 0x2b4;
/// `hkMemoryRouter` lookup (`00c85750`): its `+0x10` is the allocator whose slot `+4`
/// allocates `(bytes)` and whose slot `+0x10` frees `(pointer, bytes)`.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The game heap's allocator (`00aa13e0`, cdecl `(size)`).
const HEAP_ALLOCATE: u32 = 0x00aa_13e0;
/// `MemoryManager` allocation (`00aa3e40`) and free (`00aa4060`): `this` = the manager at
/// `011f6238`.
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_FREE: u32 = 0x00aa_4060;
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `bhkRefObject` constructor (`00c87c30`) and `bhkSerializable::Kill` (Xbox PDB).
const REF_OBJECT_CONSTRUCT: u32 = 0x00c8_7c30;
const KILL: u32 = 0x00c8_76a0;
/// `bhkConstraintChain` functions (Xbox PDB): destructor, `CopyCinfo` (`(this, cinfo)`),
/// `CopyMembers` (`(this, clone, process)`) and, unnamed, the cinfo save (`(this, stream)`).
const CHAIN_DESTRUCTOR: u32 = 0x00d5_3220;
const CHAIN_COPY_CINFO: u32 = 0x00d5_3900;
const CHAIN_COPY_MEMBERS: u32 = 0x00d5_3a80;
const CHAIN_CINFO_SAVE_BINARY: u32 = 0x00d5_3360;
/// `hkConstraintChainCinfo::LoadBinary` (Xbox PDB, `(this, stream)`).
const CHAIN_CINFO_LOAD_BINARY: u32 = 0x00d5_3200;
/// `hkpBallSocketChainData` (Havok SDK): the constructor (`this` = the block) and the unnamed
/// `00d53d30` that adds a pair of constraint vectors (`this`, `(first, second)`).
const CHAIN_DATA_CONSTRUCT: u32 = 0x00d5_3d90;
const CHAIN_DATA_ADD_PAIR: u32 = 0x00d5_3d30;
/// `bhkStream::LoadBinary` / `bhkStream::SaveBinary` (Xbox PDB): `(stream, array)` of 16-byte
/// vectors, cdecl.
const STREAM_LOAD_VECTORS: u32 = 0x00cf_0f40;
const STREAM_SAVE_VECTORS: u32 = 0x00cf_0cf0;
/// `hkArrayUtil::_reserve` (Xbox PDB): cdecl `(allocator, array, capacity, element size)`.
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `hkConstraintCinfo::CloneData` (Xbox PDB): `(this, data, cinfo)`.
const CONSTRAINT_CINFO_CLONE_DATA: u32 = 0x00c8_a1b0;
/// `hkConstraintCinfo::SetData` (Xbox PDB): `(this, data)`.
const CONSTRAINT_CINFO_SET_DATA: u32 = 0x00c8_9ac0;
/// The jump chain `00cc5080` -> `00c8a560` that `GetViewerStrings` goes through.
const VIEWER_STRINGS_TARGET: u32 = 0x00cc_5080;
/// `bhkSerializable::CloneCritLock` and `CloneCritUnlock` (Xbox PDB).
const CLONE_LOCK: u32 = 0x00c8_76f0;
const CLONE_UNLOCK: u32 = 0x00c8_7700;
/// Class vtable and the two base-class vtables the inlined constructor stores before it, with
/// the object counters.
const VTABLE_CLASS: u32 = 0x010c_a06c;
const VTABLE_CHAIN: u32 = 0x010c_a14c;
const VTABLE_BASE: u32 = 0x0108_febc;
const COUNT_CLASS: u32 = 0x0126_863c;
const COUNT_CHAIN: u32 = 0x0126_8f80;
const COUNT_BASE: u32 = 0x0126_8188;
/// The vtable of the cinfo block `CreateCinfo` builds and the vtable the array holder takes
/// when its arrays are freed.
const VTABLE_CINFO_BLOCK: u32 = 0x010c_a22c;
const VTABLE_ARRAY_HOLDER: u32 = 0x0108_ffb4;
/// Capacity words carry flags in the top two bits; this is the empty, unowned value.
const EMPTY_CAPACITY: u32 = 0x8000_0000;

/// Sets the allocation-scope word to 0x10 and returns its address and old value.
fn enter_scope(e: &mut Engine) -> (u32, u32) {
    let word = e.tls() + SCOPE_WORD;
    let saved = e.mem.u32(word);
    e.mem.set_u32(word, 0x10);
    (word, saved)
}

fn count_up(e: &mut Engine, counter: u32) {
    let value: u32 = e.global(counter);
    e.set_global(counter, value.wrapping_add(1));
}

/// The inlined `new bhkBallSocketConstraintChain`: a 0x10-byte block from the game heap, the
/// base constructors and the three counters; 0 when the allocation fails.
fn construct_chain(e: &mut Engine) -> u32 {
    let object = e.call(HEAP_ALLOCATE, &args![0x10u32]).u32();
    if object == 0 {
        return 0;
    }
    e.call(REF_OBJECT_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, VTABLE_BASE);
    e.mem.set_u32(object + 0xc, 0);
    count_up(e, COUNT_BASE);
    e.mem.set_u32(object, VTABLE_CHAIN);
    count_up(e, COUNT_CHAIN);
    e.mem.set_u32(object, VTABLE_CLASS);
    count_up(e, COUNT_CLASS);
    object
}

/// A 0x34-byte `hkpBallSocketChainData` from the Havok allocator (memory-size word at `+4` =
/// 0x34), constructed by `00d53d90`.
fn new_chain_data(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x34u32]).u32();
    e.mem.set_u16(block + 4, 0x34);
    e.call(CHAIN_DATA_CONSTRUCT, &args![block]).u32()
}

/// Grows the array at `array` (data, size, capacity-with-flags) to hold `wanted` elements of
/// `element_size` bytes when its capacity (the low 30 bits of the word at `+8`) is smaller:
/// twice the capacity, or `wanted` when that is not smaller (signed comparisons).
fn reserve_elements(e: &mut Engine, array: u32, wanted: u32, element_size: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if (capacity as i32) < (wanted as i32) {
        let doubled = capacity.wrapping_add(capacity);
        let new_capacity = if (wanted as i32) < (doubled as i32) {
            doubled
        } else {
            wanted
        };
        e.call(
            ARRAY_RESERVE,
            &args![allocator, array, new_capacity, element_size],
        );
    }
}

/// One stream transfer: calls the function at `+function` of the file at `[stream + file_slot]`
/// (`(file, buffer, size, &size, 1)`) and adds the count it answers to the file position at
/// `+4`.
fn transfer(e: &mut Engine, stream: u32, file_slot: u32, function: u32, buffer: u32, size: u32) {
    let file = e.mem.u32(stream + file_slot);
    let routine = e.mem.u32(file + function);
    let count = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), size);
        e.call(routine, &args![file, buffer, size, slot, 1u32])
            .u32()
    });
    let position = e.mem.u32(file + 4);
    e.mem.set_u32(file + 4, position.wrapping_add(count));
}

/// The four words of the chain data that the stream carries after the constraint pairs.
const CHAIN_DATA_WORDS: [u32; 4] = [0x28, 0x24, 0x2c, 0x30];

// Translated from 00cc8cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBallSocketChainCinfo::GetViewerStrings` (Xbox PDB): a jump to `00cc5080` (which jumps
/// on to `00c8a560`); the words of `(this, strings)` and the answer pass through.
pub fn hk_ball_socket_chain_cinfo_get_viewer_strings(
    e: &mut Engine,
    this: Ptr,
    strings: Ptr,
) -> u32 {
    e.call(VIEWER_STRINGS_TARGET, &args![this, strings]).u32()
}

// Translated from 00cc8ce0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc8ce0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkBallSocketConstraintChain::~bhkBallSocketConstraintChain` (Xbox PDB): stores the class
/// vtable, runs `Kill` (`00c876a0`), decrements the class counter and runs the
/// `bhkConstraintChain` destructor (`00d53220`).
pub fn bhk_ball_socket_constraint_chain_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_CLASS);
    e.call(KILL, &args![this]);
    let count: u32 = e.global(COUNT_CLASS);
    e.set_global(COUNT_CLASS, count.wrapping_sub(1));
    e.call(CHAIN_DESTRUCTOR, &args![this]);
}

// Translated from 00cc8d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBallSocketConstraintChain::QCinfoSize` (Xbox PDB): takes no stack words and returns
/// 0x2c.
pub fn bhk_ball_socket_constraint_chain_q_cinfo_size(_e: &mut Engine) -> u32 {
    0x2c
}

// Translated from 00cc8dc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc8dc0 - the FS:[0] exception-unwinding frame is not modeled.
/// Unnamed (cdecl, no arguments): `new bhkBallSocketConstraintChain`, 0 when the heap
/// returns 0.
pub fn fn_00cc8dc0(e: &mut Engine) -> u32 {
    construct_chain(e)
}

// Translated from 00cc8e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBallSocketChainCinfo::LoadBinary` (Xbox PDB): runs virtual slot 0x10 (`CreateData`),
/// has `bhkStream::LoadBinary` (`00cf0f40`, cdecl `(stream, this + 0x20)`) fill the array of
/// vectors at `+0x20`, hands each consecutive pair of its elements (`+0x24` is the size,
/// elements are 16 bytes) to `00d53d30` of the data block at `this + 4`, reads four words
/// of the data block (`+0x28`, `+0x24`, `+0x2C`, `+0x30`) through the read function of the
/// stream's file (`[stream + 0x24C]`, function at `+8`, `(file, destination, 4, &4, 1)`,
/// the count added to the file position at `+4`) and answers what `00d53200` answers on
/// `(this, stream)`.
pub fn hk_ball_socket_chain_cinfo_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) -> u32 {
    let me = this.addr();
    e.vcall(me, 0x10, &args![]);
    e.call(STREAM_LOAD_VECTORS, &args![stream, me + 0x20]);
    let data = e.mem.u32(me + 4);
    if (e.mem.u32(me + 0x24) as i32) > 0 {
        let mut index = 0i32;
        let mut offset = 0u32;
        loop {
            let first = e.mem.u32(me + 0x20).wrapping_add(offset);
            e.call(CHAIN_DATA_ADD_PAIR, &args![data, first, first + 0x10]);
            offset += 0x20;
            index += 2;
            if index >= e.mem.u32(me + 0x24) as i32 {
                break;
            }
        }
    }
    for word in CHAIN_DATA_WORDS {
        transfer(e, stream.addr(), 0x24c, 8, data + word, 4);
    }
    e.call(CHAIN_CINFO_LOAD_BINARY, &args![this, stream]).u32()
}

// Translated from 00cc8f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBallSocketChainCinfo::SaveBinary` (Xbox PDB): has `bhkStream::SaveBinary` (`00cf0cf0`,
/// cdecl `(stream, this + 0x20)`) write the array of vectors at `+0x20`, writes four words
/// of the data block at `this + 4` (`+0x28`, `+0x24`, `+0x2C`, `+0x30`) through the write
/// function of the stream's file (`[stream + 0x250]`, function at `+0xC`,
/// `(file, source, 4, &4, 1)`, the count added to the file position at `+4`) and answers
/// what `00d53360` answers on `(this, stream)`.
pub fn hk_ball_socket_chain_cinfo_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) -> u32 {
    let me = this.addr();
    let data = e.mem.u32(me + 4);
    e.call(STREAM_SAVE_VECTORS, &args![stream, me + 0x20]);
    for word in CHAIN_DATA_WORDS {
        transfer(e, stream.addr(), 0x250, 0xc, data + word, 4);
    }
    e.call(CHAIN_CINFO_SAVE_BINARY, &args![this, stream]).u32()
}

// Translated from 00cc9020 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc9020 - the FS:[0] exception-unwinding frame is not modeled.
/// `hkBallSocketChainCinfo::CreateData` (Xbox PDB) with the allocation scope word set to
/// 0x10: when `this + 4` is 0, builds a 0x34-byte `hkpBallSocketChainData` (`00d53d90`)
/// there. Answers 0x34.
pub fn hk_ball_socket_chain_cinfo_create_data(e: &mut Engine, this: Ptr) -> u32 {
    let (word, saved) = enter_scope(e);
    if e.mem.u32(this.addr() + 4) == 0 {
        let data = new_chain_data(e);
        e.mem.set_u32(this.addr() + 4, data);
    }
    e.mem.set_u32(word, saved);
    0x34
}

// Translated from 00cc90d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc90d0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkBallSocketConstraintChain::CreateClone` (Xbox PDB) with the allocation scope word set
/// to 0x10 and the clone lock held (`00c876f0`, `00c87700`): builds a new
/// `bhkBallSocketConstraintChain` and runs `bhkConstraintChain::CopyMembers` (`00d53a80`) on
/// `(this, clone, process)` (with 0 when the allocation failed). Answers the clone.
pub fn bhk_ball_socket_constraint_chain_create_clone(
    e: &mut Engine,
    this: Ptr,
    process: u32,
) -> u32 {
    let (word, saved) = enter_scope(e);
    e.call(CLONE_LOCK, &args![]);
    let clone = construct_chain(e);
    e.call(CHAIN_COPY_MEMBERS, &args![this, clone, process]);
    e.call(CLONE_UNLOCK, &args![]);
    e.mem.set_u32(word, saved);
    clone
}

// Translated from 00cc91c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBallSocketConstraintChain::CopyCinfo` (Xbox PDB): runs `bhkConstraintChain::CopyCinfo`
/// (`00d53900`) on `(this, out)`, clears `out + 0x24`, then takes the constraint data
/// (`+0xC` of the Havok constraint at `this + 8`; the exe does not test the Havok constraint
/// for 0) and copies its `count` pairs of 16-byte vectors (array at `+0x18`, 0x20-byte
/// elements, count at `+0x1C`) into the array at `out + 0x20` (data, size at `+4`, capacity
/// at `+8`), after growing it to `2 * count` elements of 16 bytes.
pub fn bhk_ball_socket_constraint_chain_copy_cinfo(e: &mut Engine, this: Ptr, out: Ptr) {
    e.call(CHAIN_COPY_CINFO, &args![this, out]);
    let constraint = if this.addr() == 0 {
        0
    } else {
        e.mem.u32(this.addr() + 8)
    };
    let array = out.addr() + 0x20;
    e.mem.set_u32(out.addr() + 0x24, 0);
    let source = e.mem.u32(constraint + 0xc);
    let count = e.mem.u32(source + 0x1c);
    let wanted = count.wrapping_add(count);
    reserve_elements(e, array, wanted, 0x10);
    e.mem.set_u32(array + 4, wanted);
    let mut index = 0i32;
    while index < e.mem.u32(source + 0x1c) as i32 {
        let from = e.mem.u32(source + 0x18) + 0x20 * index as u32;
        let to = e.mem.u32(array) + 0x20 * index as u32;
        for lane in 0..4u32 {
            let value = e.mem.u32(from + 4 * lane);
            e.mem.set_u32(to + 4 * lane, value);
        }
        for lane in 0..4u32 {
            let value = e.mem.u32(from + 0x10 + 4 * lane);
            e.mem.set_u32(to + 0x10 + 4 * lane, value);
        }
        index += 1;
    }
}

// Translated from 00cc9270 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc9270 - the FS:[0] exception-unwinding frame is not modeled.
/// `hkBallSocketChainCinfo::CloneData` (Xbox PDB) with the allocation scope word set to
/// 0x10: `data` is the destination (a new 0x34-byte `hkpBallSocketChainData`, `00d53d90`,
/// when it is 0). Copies the array of 0x20-byte elements at `+0x18` of the data block at
/// `this + 4` (count at `+0x1C`) into the array at `+0x18` of the destination (grown first),
/// scales the eight floats of every element by the float at `info + 0x10` (`FLD`/`FSTP`
/// copy) unless that equals 1.0, and answers what `hkConstraintCinfo::CloneData`
/// (`00c8a1b0`) answers on `(this, data, info)`.
pub fn hk_ball_socket_chain_cinfo_clone_data(
    e: &mut Engine,
    this: Ptr,
    data: Ptr,
    info: Ptr,
) -> u32 {
    let (word, saved) = enter_scope(e);
    let source = e.mem.u32(this.addr() + 4);
    let destination = if data.addr() == 0 {
        new_chain_data(e)
    } else {
        data.addr()
    };
    let wanted = e.mem.u32(source + 0x1c);
    let array = destination + 0x18;
    reserve_elements(e, array, wanted, 0x20);
    e.mem.set_u32(array + 4, wanted);
    let count = e.mem.u32(source + 0x1c);
    let from = e.mem.u32(source + 0x18);
    let to = e.mem.u32(array);
    if (count as i32) > 0 {
        for index in 0..count {
            for lane in 0..8u32 {
                let value = e.mem.u32(from + 0x20 * index + 4 * lane);
                e.mem.set_u32(to + 0x20 * index + 4 * lane, value);
            }
        }
    }
    let scale = x87_quiet(e.mem.f32(info.addr() + 0x10));
    if scale != 1.0 {
        let mut index = 0u32;
        while (index as i32) < e.mem.u32(destination + 0x1c) as i32 {
            let element = e.mem.u32(array) + 0x20 * index;
            for lane in 0..8u32 {
                let address = element + 4 * lane;
                let value = e.mem.f32(address);
                e.mem.set_f32(address, scale * value);
            }
            index += 1;
        }
    }
    let answer = e
        .call(CONSTRAINT_CINFO_CLONE_DATA, &args![this, destination, info])
        .u32();
    e.mem.set_u32(word, saved);
    answer
}

// Translated from 00cc9410 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc9410 - the FS:[0] exception-unwinding frame is not modeled.
/// Unnamed (`this`): clears the size at `+0x18`; when the capacity word at `+0x1C` is not
/// negative, frees the array at `+0x14` (`(pointer, 4 * capacity)` through slot `+0x10` of
/// the Havok allocator); then clears `+0x14`, sets `+0x1C` to 0x80000000, stores the vtable
/// `0108ffb4` and runs `hkConstraintCinfo::SetData` (`00c89ac0`) with 0.
pub fn fn_00cc9410(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(me + 0x1c);
    e.mem.set_u32(me + 0x18, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(me + 0x14);
        e.vcall(allocator, 0x10, &args![data, (capacity & 0x3fff_ffff) << 2]);
    }
    e.mem.set_u32(me + 0x14, 0);
    e.mem.set_u32(me + 0x1c, EMPTY_CAPACITY);
    e.mem.set_u32(me, VTABLE_ARRAY_HOLDER);
    e.call(CONSTRAINT_CINFO_SET_DATA, &args![this, 0u32]);
}

// Translated from 00cc94b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc94b0 - the FS:[0] exception-unwinding frame is not modeled.
/// Unnamed (`this`): clears the size at `+0x24`; when the capacity word at `+0x28` is not
/// negative, frees the array at `+0x20` (`(pointer, 16 * capacity)` through slot `+0x10` of
/// the Havok allocator); then clears `+0x20`, sets `+0x28` to 0x80000000 and runs `00cc9410`.
pub fn fn_00cc94b0(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(me + 0x28);
    e.mem.set_u32(me + 0x24, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(me + 0x20);
        e.vcall(allocator, 0x10, &args![data, (capacity & 0x3fff_ffff) << 4]);
    }
    e.mem.set_u32(me + 0x20, 0);
    e.mem.set_u32(me + 0x28, EMPTY_CAPACITY);
    fn_00cc9410(e, this);
}

// Translated from 00cc9540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBallSocketChainCinfo::_scalar_deleting_destructor_` (Xbox PDB): runs `00cc94b0`, then
/// frees `this` through the memory manager (`00aa4060`, `this` = the manager at `011f6238`)
/// when bit 0 of `flags` is set. Answers `this`.
pub fn hk_ball_socket_chain_cinfo_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00cc94b0(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00cc9560 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc9560 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkBallSocketConstraintChain::CreateCinfo` (Xbox PDB), with the allocation scope word
/// set to 0x10: stores in the byte at `created` whether the cinfo at `+0xC` was missing; then
/// builds it: a 0x2c-byte block from the memory manager (`00aa3e40`) holding the vtable
/// `010ca22c`, `+4` = 0, `+8` = 1, `+0xC`, `+0x10`, `+0x14`, `+0x18` = 0, `+0x1C` and `+0x28`
/// = 0x80000000, `+0x20` and `+0x24` = 0; the cinfo is the address `block + 4` (0 when the
/// manager answers 0), stored at `+0xC` and, when `+8` is not 0, filled by
/// `bhkBallSocketConstraintChain::CopyCinfo` (`00cc91c0`) given the block (0 for no cinfo).
/// Answers the cinfo.
pub fn bhk_ball_socket_constraint_chain_create_cinfo(
    e: &mut Engine,
    this: Ptr,
    created: Ptr,
) -> u32 {
    let (word, saved) = enter_scope(e);
    let missing = e.mem.u32(this.addr() + 0xc) == 0;
    e.mem.set_u8(created.addr(), u8::from(missing));
    if missing {
        let block = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x2cu32])
            .u32();
        let cinfo = if block == 0 {
            0
        } else {
            e.mem.set_u32(block + 4, 0);
            e.mem.set_u32(block + 8, 1);
            e.mem.set_u32(block + 0xc, 0);
            e.mem.set_u32(block + 0x10, 0);
            e.mem.set_u32(block + 0x14, 0);
            e.mem.set_u32(block + 0x18, 0);
            e.mem.set_u32(block + 0x1c, EMPTY_CAPACITY);
            e.mem.set_u32(block, VTABLE_CINFO_BLOCK);
            e.mem.set_u32(block + 0x20, 0);
            e.mem.set_u32(block + 0x24, 0);
            e.mem.set_u32(block + 0x28, EMPTY_CAPACITY);
            block + 4
        };
        e.mem.set_u32(this.addr() + 0xc, cinfo);
        if e.mem.u32(this.addr() + 8) != 0 {
            let argument = if cinfo == 0 { 0 } else { cinfo - 4 };
            bhk_ball_socket_constraint_chain_copy_cinfo(e, this, Ptr::new(argument));
        }
    }
    let cinfo = e.mem.u32(this.addr() + 0xc);
    e.mem.set_u32(word, saved);
    cinfo
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cc8d50, bhk_ball_socket_constraint_chain_get_r_t_t_i() -> u32),
        entry!(0x00cc8d70, bhk_constraint_chain_get_r_t_t_i() -> u32),
        entry!(0x00cc8d80, bhk_constraint_chain_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00cc8da0, bhk_ball_socket_constraint_chain_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00cc8cd0, hk_ball_socket_chain_cinfo_get_viewer_strings(Ptr, Ptr) -> u32),
        entry!(0x00cc8ce0, bhk_ball_socket_constraint_chain_destructor(Ptr)),
        entry!(0x00cc8d60, bhk_ball_socket_constraint_chain_q_cinfo_size() -> u32),
        entry!(0x00cc8dc0, fn_00cc8dc0() -> u32),
        entry!(0x00cc8e60, hk_ball_socket_chain_cinfo_load_binary(Ptr, Ptr) -> u32),
        entry!(0x00cc8f60, hk_ball_socket_chain_cinfo_save_binary(Ptr, Ptr) -> u32),
        entry!(0x00cc9020, hk_ball_socket_chain_cinfo_create_data(Ptr) -> u32),
        entry!(0x00cc90d0, bhk_ball_socket_constraint_chain_create_clone(Ptr, u32) -> u32),
        entry!(
            0x00cc91c0,
            bhk_ball_socket_constraint_chain_copy_cinfo(Ptr, Ptr)
        ),
        entry!(0x00cc9270, hk_ball_socket_chain_cinfo_clone_data(Ptr, Ptr, Ptr) -> u32),
        entry!(0x00cc9410, fn_00cc9410(Ptr)),
        entry!(0x00cc94b0, fn_00cc94b0(Ptr)),
        entry!(0x00cc9540, hk_ball_socket_chain_cinfo_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00cc9560, bhk_ball_socket_constraint_chain_create_cinfo(Ptr, Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bhk_constraint_chain_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00d5_3220, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00cc8d80, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00cc8d80, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x10]]);
        assert_eq!(calls_to(&e, 0x00d5_3220), vec![vec![this]; 2]);
    }

    #[test]
    fn bhk_ball_socket_constraint_chain_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00cc_8ce0, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00cc8da0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00cc8da0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x10]]);
        assert_eq!(calls_to(&e, 0x00cc_8ce0), vec![vec![this]; 2]);
    }

    #[test]
    fn bhk_ball_socket_constraint_chain_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cc_8d50, &args![]).u32(), 0x0126_8640);
    }

    #[test]
    fn bhk_constraint_chain_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cc_8d70, &args![]).u32(), 0x0126_8f84);
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn map_counters(e: &mut Engine) {
        for counter in [COUNT_BASE, COUNT_CHAIN, COUNT_CLASS] {
            e.map(counter, 4);
            e.set_global(counter, 10u32);
        }
    }

    /// A vtable of `slots` (byte offset, target) in front of a 0x40-byte object.
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let table = e.mem.alloc(0x100);
        for (offset, target) in slots {
            e.mem.set_u32(table + offset, *target);
        }
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, table);
        object
    }

    /// Makes the memory router answer an allocator (an object) whose slot 4 returns `block`
    /// (recording the requested size at `block + 0x48`) and whose slot 0x10 is the double
    /// `0x0900_0010`. Answers the allocator.
    fn route_allocation(e: &mut Engine, block: u32) -> u32 {
        let allocator = object(e, &[(4, 0x0900_0004), (0x10, 0x0900_0010)]);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register_double(0x0900_0004, move |e, a| {
            e.mem.set_u32(block + 0x48, a[1]);
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        e.register(0x0900_0010, nothing);
        allocator
    }

    #[test]
    fn get_viewer_strings_jumps_to_the_prismatic_target() {
        let mut e = Engine::new();
        e.register(VIEWER_STRINGS_TARGET, |_, a| Ret {
            eax: a[0] + a[1],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00cc8cd0, &args![0x100u32, 0x20u32]).u32(), 0x120);
        assert_eq!(calls(&e, VIEWER_STRINGS_TARGET), [[0x100, 0x20]]);
    }

    #[test]
    fn destructor_stores_the_vtable_kills_and_destroys_the_base() {
        let mut e = Engine::new();
        map_counters(&mut e);
        e.register(KILL, nothing);
        e.register(CHAIN_DESTRUCTOR, nothing);
        let this = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        e.call(0x00cc8ce0, &args![this]);
        let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(order, [0x00cc8ce0, KILL, CHAIN_DESTRUCTOR]);
        assert_eq!(e.mem.u32(this), VTABLE_CLASS);
        assert_eq!(e.global::<u32>(COUNT_CLASS), 9);
    }

    #[test]
    fn q_cinfo_size_is_a_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cc8d60, &args![]).u32(), 0x2c);
    }

    #[test]
    fn new_builds_the_object_and_counts_it_or_answers_zero() {
        let mut e = Engine::new();
        map_counters(&mut e);
        let block = e.mem.alloc(0x10);
        e.register_double(HEAP_ALLOCATE, move |_, _| Ret {
            eax: block,
            ..Ret::default()
        });
        e.register(REF_OBJECT_CONSTRUCT, nothing);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00cc8dc0, &args![]).u32(), block);
        assert_eq!(calls(&e, HEAP_ALLOCATE), [[0x10]]);
        assert_eq!(calls(&e, REF_OBJECT_CONSTRUCT), [[block]]);
        assert_eq!(e.mem.u32(block), VTABLE_CLASS);
        assert_eq!(e.mem.u32(block + 0xc), 0);
        for counter in [COUNT_BASE, COUNT_CHAIN, COUNT_CLASS] {
            assert_eq!(e.global::<u32>(counter), 11);
        }
        e.register(HEAP_ALLOCATE, nothing);
        assert_eq!(e.call(0x00cc8dc0, &args![]).u32(), 0);
        assert_eq!(e.global::<u32>(COUNT_CLASS), 11);
    }

    /// A cinfo wrapper with a data block at `+4` and `count` vectors at `+0x20`; a stream
    /// whose file (read at `+0x24C`, write at `+0x250`) has the read function 0x0900_0040
    /// and the write function 0x0900_0044.
    fn stream_setup(e: &mut Engine, count: u32) -> (u32, u32, u32, u32) {
        let this = object(e, &[(0x10, 0x0900_0010)]);
        let data = e.mem.alloc(0x40);
        let vectors = e.mem.alloc(0x100);
        e.mem.set_u32(this + 4, data);
        e.mem.set_u32(this + 0x20, vectors);
        e.mem.set_u32(this + 0x24, count);
        let file = object(e, &[]);
        e.mem.set_u32(file + 8, 0x0900_0040);
        e.mem.set_u32(file + 0xc, 0x0900_0044);
        e.mem.set_u32(file + 4, 100);
        let stream = e.mem.alloc(0x300);
        e.mem.set_u32(stream + 0x24c, file);
        e.mem.set_u32(stream + 0x250, file);
        (this, data, stream, file)
    }

    #[test]
    fn load_binary_hands_pairs_to_the_data_and_reads_four_words() {
        for (count, pairs) in [(5u32, 3usize), (2, 1), (0, 0), (0xffff_ffff, 0)] {
            let mut e = Engine::new();
            let (this, data, stream, file) = stream_setup(&mut e, count);
            let vectors = e.mem.u32(this + 0x20);
            e.register(0x0900_0010, nothing);
            e.register(STREAM_LOAD_VECTORS, nothing);
            e.register(CHAIN_DATA_ADD_PAIR, nothing);
            e.register(CHAIN_CINFO_LOAD_BINARY, |_, _| Ret {
                eax: 0x77,
                ..Ret::default()
            });
            e.register(0x0900_0040, |e, a| {
                assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
                e.mem.set_u32(a[1], a[1]);
                Ret {
                    eax: 4,
                    ..Ret::default()
                }
            });
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00cc8e60, &args![this, stream]).u32(), 0x77);
            let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
            assert_eq!(order[..3], [0x00cc8e60, 0x0900_0010, STREAM_LOAD_VECTORS]);
            assert_eq!(calls(&e, STREAM_LOAD_VECTORS), [[stream, this + 0x20]]);
            let added = calls(&e, CHAIN_DATA_ADD_PAIR);
            assert_eq!(added.len(), pairs);
            for (index, call) in added.iter().enumerate() {
                let first = vectors + 0x20 * index as u32;
                assert_eq!(call, &[data, first, first + 0x10]);
            }
            let reads: Vec<u32> = calls(&e, 0x0900_0040).iter().map(|c| c[1]).collect();
            assert_eq!(reads, [data + 0x28, data + 0x24, data + 0x2c, data + 0x30]);
            assert_eq!(e.mem.u32(file + 4), 116);
            assert_eq!(calls(&e, CHAIN_CINFO_LOAD_BINARY), [[this, stream]]);
        }
    }

    #[test]
    fn save_binary_writes_the_vectors_then_four_words() {
        let mut e = Engine::new();
        let (this, data, stream, file) = stream_setup(&mut e, 2);
        e.register(STREAM_SAVE_VECTORS, nothing);
        e.register(CHAIN_CINFO_SAVE_BINARY, |_, _| Ret {
            eax: 0x55,
            ..Ret::default()
        });
        e.register(0x0900_0044, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            Ret {
                eax: 4,
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00cc8f60, &args![this, stream]).u32(), 0x55);
        let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(order[..2], [0x00cc8f60, STREAM_SAVE_VECTORS]);
        assert_eq!(order[order.len() - 1], CHAIN_CINFO_SAVE_BINARY);
        assert_eq!(calls(&e, STREAM_SAVE_VECTORS), [[stream, this + 0x20]]);
        let writes: Vec<u32> = calls(&e, 0x0900_0044).iter().map(|c| c[1]).collect();
        assert_eq!(writes, [data + 0x28, data + 0x24, data + 0x2c, data + 0x30]);
        assert_eq!(e.mem.u32(file + 4), 116);
        assert_eq!(calls(&e, CHAIN_CINFO_SAVE_BINARY), [[this, stream]]);
    }

    #[test]
    fn create_data_builds_the_data_block_once() {
        let mut e = Engine::new();
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 0x55);
        let block = e.mem.alloc(0x60);
        route_allocation(&mut e, block);
        e.register(CHAIN_DATA_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00cc9020, &args![this]).u32(), 0x34);
        assert_eq!(e.mem.u32(this + 4), block);
        assert_eq!(e.mem.u16(block + 4), 0x34);
        assert_eq!(e.mem.u32(block + 0x48), 0x34);
        assert_eq!(e.call(0x00cc9020, &args![this]).u32(), 0x34);
        assert_eq!(calls(&e, CHAIN_DATA_CONSTRUCT).len(), 1);
        assert_eq!(e.mem.u32(word), 0x55);
    }

    #[test]
    fn create_clone_copies_members_between_the_clone_lock_calls() {
        let mut e = Engine::new();
        map_counters(&mut e);
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 0x55);
        let block = e.mem.alloc(0x10);
        e.register_double(HEAP_ALLOCATE, move |_, _| Ret {
            eax: block,
            ..Ret::default()
        });
        for address in [
            REF_OBJECT_CONSTRUCT,
            CLONE_LOCK,
            CLONE_UNLOCK,
            CHAIN_COPY_MEMBERS,
        ] {
            e.register(address, nothing);
        }
        let this = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00cc90d0, &args![this, 0x7700u32]).u32(), block);
        let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            [
                0x00cc90d0,
                CLONE_LOCK,
                HEAP_ALLOCATE,
                REF_OBJECT_CONSTRUCT,
                CHAIN_COPY_MEMBERS,
                CLONE_UNLOCK
            ]
        );
        assert_eq!(calls(&e, CHAIN_COPY_MEMBERS), [[this, block, 0x7700]]);
        assert_eq!(e.mem.u32(block), VTABLE_CLASS);
        assert_eq!(e.mem.u32(word), 0x55);
    }

    /// A constraint chain whose Havok constraint's data holds `count` elements of eight words
    /// (element `i`, lane `l` = `0x100 * i + l`), and an output cinfo whose array at `+0x20`
    /// has the capacity word `capacity`. The reserve function is a double that gives the
    /// array a fresh buffer.
    fn copy_setup(e: &mut Engine, count: u32, capacity: u32) -> (u32, u32, u32, u32) {
        let allocator = route_allocation(e, 0);
        let this = e.mem.alloc(0x20);
        let constraint = e.mem.alloc(0x20);
        let source = e.mem.alloc(0x40);
        let elements = e.mem.alloc(0x20 * count.max(1));
        for index in 0..count {
            for lane in 0..8u32 {
                e.mem
                    .set_u32(elements + 0x20 * index + 4 * lane, 0x100 * index + lane);
            }
        }
        e.mem.set_u32(this + 8, constraint);
        e.mem.set_u32(constraint + 0xc, source);
        e.mem.set_u32(source + 0x18, elements);
        e.mem.set_u32(source + 0x1c, count);
        let out = e.mem.alloc(0x40);
        e.mem.set_u32(out + 0x24, 0x1234);
        e.mem.set_u32(out + 0x28, capacity);
        e.register(CHAIN_COPY_CINFO, nothing);
        let buffer = e.mem.alloc(0x20 * count.max(1) + 0x20);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            e.mem.set_u32(a[1], buffer);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        e.mem.set_u32(out + 0x20, buffer);
        (this, out, buffer, allocator)
    }

    #[test]
    fn copy_cinfo_grows_the_array_and_copies_the_pairs() {
        // (count, capacity word, requested capacity or none)
        let cases = [
            (3u32, 0x8000_0000u32, Some(6u32)),
            (2, 3, Some(6)),
            (4, 1, Some(8)),
            (2, 4, None),
            (2, 0x4000_0004, None),
            (0, 0x8000_0000, None),
        ];
        for (count, capacity, requested) in cases {
            let mut e = Engine::new();
            let (this, out, buffer, allocator) = copy_setup(&mut e, count, capacity);
            e.call_log = Some(vec![]);
            e.call(0x00cc91c0, &args![this, out]);
            assert_eq!(calls(&e, CHAIN_COPY_CINFO), [[this, out]]);
            let reserves = calls(&e, ARRAY_RESERVE);
            match requested {
                Some(size) => {
                    assert_eq!(reserves.len(), 1);
                    assert_eq!(reserves[0], [allocator, out + 0x20, size, 0x10]);
                }
                None => assert!(reserves.is_empty()),
            }
            assert_eq!(e.mem.u32(out + 0x24), 2 * count);
            assert_eq!(e.mem.u32(out + 0x20), buffer);
            for index in 0..count {
                for lane in 0..8u32 {
                    assert_eq!(
                        e.mem.u32(buffer + 0x20 * index + 4 * lane),
                        0x100 * index + lane
                    );
                }
            }
        }
    }

    #[test]
    fn copy_cinfo_copies_into_an_existing_buffer_without_growing() {
        let mut e = Engine::new();
        let (this, out, _, _) = copy_setup(&mut e, 2, 8);
        let existing = e.mem.alloc(0x80);
        e.mem.set_u32(out + 0x20, existing);
        e.call(0x00cc91c0, &args![this, out]);
        assert_eq!(e.mem.u32(existing + 0x20), 0x100);
        assert_eq!(e.mem.u32(existing + 0x3c), 0x107);
        assert_eq!(e.mem.u32(existing), 0);
        assert_eq!(e.mem.u32(existing + 0x1c), 7);
    }

    #[test]
    fn clone_data_copies_and_scales_the_elements_into_the_given_data() {
        for (scale, factor) in [(1.0f32, 1.0f32), (2.0, 2.0)] {
            let mut e = Engine::new();
            let word = e.tls() + SCOPE_WORD;
            e.mem.set_u32(word, 0x55);
            let (this_source, _, buffer, _) = copy_setup(&mut e, 2, 0x8000_0000);
            // The wrapper holds the data block at +4, whose array is what is cloned.
            let wrapper = e.mem.alloc(0x20);
            let source_data = e.mem.u32(e.mem.u32(this_source + 8) + 0xc);
            e.mem.set_u32(wrapper + 4, source_data);
            for index in 0..2u32 {
                for lane in 0..8u32 {
                    let address = e.mem.u32(source_data + 0x18) + 0x20 * index + 4 * lane;
                    e.mem.set_f32(address, 1.0 + (8 * index + lane) as f32);
                }
            }
            let destination = e.mem.alloc(0x40);
            let info = e.mem.alloc(0x20);
            e.mem.set_f32(info + 0x10, scale);
            e.register_double(CONSTRAINT_CINFO_CLONE_DATA, |e, _| {
                let scope = e.tls() + SCOPE_WORD;
                assert_eq!(e.mem.u32(scope), 0x10);
                Ret {
                    eax: 0xc10e,
                    ..Ret::default()
                }
            });
            e.call_log = Some(vec![]);
            let answer = e.call(0x00cc9270, &args![wrapper, destination, info]).u32();
            assert_eq!(answer, 0xc10e);
            assert_eq!(e.mem.u32(destination + 0x1c), 2);
            assert_eq!(e.mem.u32(destination + 0x18), buffer);
            for index in 0..2u32 {
                for lane in 0..8u32 {
                    assert_eq!(
                        e.mem.f32(buffer + 0x20 * index + 4 * lane),
                        factor * (1.0 + (8 * index + lane) as f32)
                    );
                }
            }
            assert_eq!(
                calls(&e, ARRAY_RESERVE)[0][1..],
                [destination + 0x18, 2, 0x20]
            );
            assert_eq!(
                calls(&e, CONSTRAINT_CINFO_CLONE_DATA),
                [[wrapper, destination, info]]
            );
            assert_eq!(e.mem.u32(word), 0x55);
        }
    }

    #[test]
    fn clone_data_builds_a_new_data_block_when_none_is_given() {
        let mut e = Engine::new();
        let (this_source, _, _, _) = copy_setup(&mut e, 1, 0x8000_0000);
        let wrapper = e.mem.alloc(0x20);
        let source_data = e.mem.u32(e.mem.u32(this_source + 8) + 0xc);
        e.mem.set_u32(wrapper + 4, source_data);
        let block = e.mem.alloc(0x60);
        route_allocation(&mut e, block);
        e.register(CHAIN_DATA_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(CONSTRAINT_CINFO_CLONE_DATA, nothing);
        let info = e.mem.alloc(0x20);
        e.mem.set_f32(info + 0x10, 1.0);
        e.call_log = Some(vec![]);
        e.call(0x00cc9270, &args![wrapper, 0u32, info]);
        assert_eq!(e.mem.u32(block + 0x48), 0x34);
        assert_eq!(e.mem.u16(block + 4), 0x34);
        assert_eq!(calls(&e, CHAIN_DATA_CONSTRUCT), [[block]]);
        assert_eq!(e.mem.u32(block + 0x1c), 1);
        assert_eq!(
            calls(&e, CONSTRAINT_CINFO_CLONE_DATA),
            [[wrapper, block, info]]
        );
    }

    #[test]
    fn fn_00cc9410_frees_the_owned_array_and_resets_the_holder() {
        for (capacity, freed) in [
            (0x0000_0010u32, true),
            (0x4000_0003, true),
            (0x8000_0010, false),
        ] {
            let mut e = Engine::new();
            let allocator = route_allocation(&mut e, 0);
            let this = e.mem.alloc(0x40);
            e.mem.set_u32(this + 0x14, 0x7000);
            e.mem.set_u32(this + 0x18, 5);
            e.mem.set_u32(this + 0x1c, capacity);
            e.register(CONSTRAINT_CINFO_SET_DATA, nothing);
            e.call_log = Some(vec![]);
            e.call(0x00cc9410, &args![this]);
            let frees = calls(&e, 0x0900_0010);
            if freed {
                assert_eq!(frees, [[allocator, 0x7000, (capacity & 0x3fff_ffff) * 4]]);
            } else {
                assert!(frees.is_empty());
            }
            assert_eq!(e.mem.u32(this + 0x14), 0);
            assert_eq!(e.mem.u32(this + 0x18), 0);
            assert_eq!(e.mem.u32(this + 0x1c), 0x8000_0000);
            assert_eq!(e.mem.u32(this), VTABLE_ARRAY_HOLDER);
            assert_eq!(calls(&e, CONSTRAINT_CINFO_SET_DATA), [[this, 0]]);
        }
    }

    #[test]
    fn fn_00cc94b0_frees_the_vector_array_then_the_holder_arrays() {
        let mut e = Engine::new();
        let allocator = route_allocation(&mut e, 0);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x20, 0x7000);
        e.mem.set_u32(this + 0x24, 5);
        e.mem.set_u32(this + 0x28, 0x0000_0006);
        e.mem.set_u32(this + 0x1c, 0x8000_0000);
        e.register(CONSTRAINT_CINFO_SET_DATA, nothing);
        e.call_log = Some(vec![]);
        e.call(0x00cc94b0, &args![this]);
        assert_eq!(calls(&e, 0x0900_0010), [[allocator, 0x7000, 6 * 16]]);
        assert_eq!(e.mem.u32(this + 0x20), 0);
        assert_eq!(e.mem.u32(this + 0x24), 0);
        assert_eq!(e.mem.u32(this + 0x28), 0x8000_0000);
        assert_eq!(e.mem.u32(this), VTABLE_ARRAY_HOLDER);
        assert_eq!(calls(&e, CONSTRAINT_CINFO_SET_DATA).len(), 1);
        // An unowned array is not freed.
        e.mem.set_u32(this + 0x28, 0x8000_0006);
        e.call(0x00cc94b0, &args![this]);
        assert_eq!(calls(&e, 0x0900_0010).len(), 1);
    }

    #[test]
    fn scalar_deleting_destructor_frees_the_memory_only_on_bit_0() {
        let mut e = Engine::new();
        route_allocation(&mut e, 0);
        e.register(CONSTRAINT_CINFO_SET_DATA, nothing);
        e.register(MEMORY_FREE, nothing);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x28, 0x8000_0000);
        e.mem.set_u32(this + 0x1c, 0x8000_0000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00cc9540, &args![this, 0xfffeu32]).u32(), this);
        assert!(calls(&e, MEMORY_FREE).is_empty());
        assert_eq!(e.call(0x00cc9540, &args![this, 1u32]).u32(), this);
        assert_eq!(calls(&e, MEMORY_FREE), [[MEMORY_MANAGER, this]]);
    }

    #[test]
    fn create_cinfo_builds_the_block_once_and_fills_it_when_the_havok_object_exists() {
        let mut e = Engine::new();
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 0x55);
        let block = e.mem.alloc(0x2c);
        e.register_double(MEMORY_ALLOCATE, move |_, _| Ret {
            eax: block,
            ..Ret::default()
        });
        let (this, _, _, _) = copy_setup(&mut e, 1, 0x8000_0000);
        e.mem.set_u32(this + 0xc, 0);
        let created = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00cc9560, &args![this, created]).u32(), block + 4);
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(calls(&e, MEMORY_ALLOCATE), [[MEMORY_MANAGER, 0x2c]]);
        assert_eq!(e.mem.u32(block), VTABLE_CINFO_BLOCK);
        assert_eq!(e.mem.u32(block + 8), 1);
        assert_eq!(e.mem.u32(this + 0xc), block + 4);
        assert_eq!(calls(&e, CHAIN_COPY_CINFO), [[this, block]]);
        // Second time: nothing is built.
        assert_eq!(e.call(0x00cc9560, &args![this, created]).u32(), block + 4);
        assert_eq!(e.mem.u8(created), 0);
        assert_eq!(calls(&e, MEMORY_ALLOCATE).len(), 1);
        assert_eq!(e.mem.u32(word), 0x55);
        // No Havok constraint: no copy.
        let bare = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00cc9560, &args![bare, created]).u32(), block + 4);
        assert_eq!(calls(&e, CHAIN_COPY_CINFO).len(), 1);
        // The block built without a Havok constraint keeps its initial words.
        assert_eq!(e.mem.u32(block + 0x1c), 0x8000_0000);
        assert_eq!(e.mem.u32(block + 0x20), 0);
        assert_eq!(e.mem.u32(block + 0x24), 0);
        assert_eq!(e.mem.u32(block + 0x28), 0x8000_0000);
    }
}
