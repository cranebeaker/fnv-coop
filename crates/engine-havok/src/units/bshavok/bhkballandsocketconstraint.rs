//! `BSHavok/bhkballandsocketconstraint.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkBallAndSocketConstraint` (Xbox PDB) is a `bhkConstraint` (0x10 bytes: `+8` the Havok
//! constraint, `+0xC` the cinfo block). Its cinfo wrapper (`hkBallAndSocketConstraintCinfo`
//! in the Xbox PDB) keeps the `hkpBallAndSocketConstraintData` at `+4`; the two vectors of
//! four floats at `+0x20` and `+0x30` of that data are what the unit copies, scales and
//! streams.
//!
//! The functions marked `Translation incomplete` carry an `FS:[0]` exception-unwinding frame
//! that is not modeled.

#[allow(unused_imports)]
use crate::prelude::*;

/// Per-thread word (`TLS + 0x2b4`) that the allocating functions set to 0x10 around their work.
const SCOPE_WORD: u32 = 0x2b4;
/// `hkMemoryRouter` lookup (`00c85750`): its `+0x10` is the allocator whose slot `+4`
/// allocates `(bytes)`.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The game heap's allocator (`00aa13e0`, cdecl `(size)`).
const HEAP_ALLOCATE: u32 = 0x00aa_13e0;
/// `MemoryManager` allocation (`00aa3e40`, `this` = the manager at `011f6238`, `(size)`).
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `bhkRefObject` constructor (`00c87c30`) and `bhkSerializable::Kill` (Xbox PDB).
const REF_OBJECT_CONSTRUCT: u32 = 0x00c8_7c30;
const KILL: u32 = 0x00c8_76a0;
/// `bhkConstraint` functions (Xbox PDB): destructor, `CopyMembers` (`(this, clone, process)`)
/// and `CopyCinfo` (`(this, cinfo)`).
const CONSTRAINT_DESTRUCTOR: u32 = 0x00c8_9640;
const CONSTRAINT_COPY_MEMBERS: u32 = 0x00c8_9730;
const CONSTRAINT_COPY_CINFO: u32 = 0x00c8_9c80;
/// `hkConstraintCinfo` functions (Xbox PDB): `CloneData` (`(this, data, cinfo)`),
/// `LoadBinary` and `SaveBinary` (`(this, stream)`).
const CONSTRAINT_CINFO_CLONE_DATA: u32 = 0x00c8_a1b0;
const CONSTRAINT_CINFO_LOAD_BINARY: u32 = 0x00c8_a230;
const CONSTRAINT_CINFO_SAVE_BINARY: u32 = 0x00c8_a280;
/// `hkpBallAndSocketConstraintData::hkpBallAndSocketConstraintData` (Xbox PDB; `this` = the
/// block).
const BALL_AND_SOCKET_DATA_CONSTRUCT: u32 = 0x00d5_4290;
/// `bhkSerializable::CloneCritLock` and `CloneCritUnlock` (Xbox PDB).
const CLONE_LOCK: u32 = 0x00c8_76f0;
const CLONE_UNLOCK: u32 = 0x00c8_7700;
/// Class vtable and the base-class vtable the inlined constructor stores before it, with the
/// object counters.
const VTABLE_CLASS: u32 = 0x010c_8b0c;
const VTABLE_BASE: u32 = 0x0108_febc;
const COUNT_CLASS: u32 = 0x0126_8648;
const COUNT_BASE: u32 = 0x0126_8188;
/// The vtable of the 0x14-byte block `CreateCinfo` builds.
const VTABLE_CINFO_BLOCK: u32 = 0x010c_8cc8;

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

/// The inlined `new bhkBallAndSocketConstraint`: a 0x10-byte block from the game heap, the
/// base constructors and the two counters; 0 when the allocation fails.
fn construct_constraint(e: &mut Engine) -> u32 {
    let object = e.call(HEAP_ALLOCATE, &args![0x10u32]).u32();
    if object == 0 {
        return 0;
    }
    e.call(REF_OBJECT_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, VTABLE_BASE);
    e.mem.set_u32(object + 0xc, 0);
    count_up(e, COUNT_BASE);
    e.mem.set_u32(object, VTABLE_CLASS);
    count_up(e, COUNT_CLASS);
    object
}

/// A 0x50-byte `hkpBallAndSocketConstraintData` from the Havok allocator (memory-size word at
/// `+4` = 0x50), constructed by `00d54290`.
fn new_constraint_data(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x50u32]).u32();
    e.mem.set_u16(block + 4, 0x50);
    e.call(BALL_AND_SOCKET_DATA_CONSTRUCT, &args![block]).u32()
}

// Translated from 00cc9640 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc9640 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkBallAndSocketConstraint::~bhkBallAndSocketConstraint` (Xbox PDB): stores the class
/// vtable, runs `Kill` (`00c876a0`), decrements the class counter and runs the
/// `bhkConstraint` destructor (`00c89640`).
pub fn bhk_ball_and_socket_constraint_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_CLASS);
    e.call(KILL, &args![this]);
    let count: u32 = e.global(COUNT_CLASS);
    e.set_global(COUNT_CLASS, count.wrapping_sub(1));
    e.call(CONSTRAINT_DESTRUCTOR, &args![this]);
}

// Translated from 00cc96b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBallAndSocketConstraintCinfo::ApplyScale` (Xbox PDB), cdecl `(scale, cinfo)` with the
/// scale as a float word: when `cinfo` is not 0 and the scale is not equal to 1.0 (a NaN
/// scale is not equal), multiplies the two vectors of four floats at `cinfo + 0x20` and
/// `cinfo + 0x30` by the scale. The exe uses aligned 16-byte loads and stores.
pub fn hk_ball_and_socket_constraint_cinfo_apply_scale(e: &mut Engine, scale: u32, cinfo: Ptr) {
    if cinfo.addr() == 0 {
        return;
    }
    let scale = f32::from_bits(scale);
    if scale == 1.0 {
        return;
    }
    for lane in 0..8u32 {
        let address = cinfo.addr() + 0x20 + 4 * lane;
        let value = e.mem.f32(address);
        e.mem.set_f32(address, crate::sse::mul(scale, value));
    }
}

// Translated from 00cc9700 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc9700 - the FS:[0] exception-unwinding frame is not modeled.
/// Unnamed (cdecl, no arguments): `new bhkBallAndSocketConstraint`, 0 when the heap returns 0.
pub fn fn_00cc9700(e: &mut Engine) -> u32 {
    construct_constraint(e)
}

// Translated from 00cc9790 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc9790 - the FS:[0] exception-unwinding frame is not modeled.
/// `hkBallAndSocketConstraintCinfo::CloneData` (Xbox PDB) with the allocation scope word set
/// to 0x10: `data` is the destination (a new 0x50-byte `hkpBallAndSocketConstraintData`,
/// `00d54290`, when it is 0); copies the two vectors at `+0x20` and `+0x30` of the data
/// block held at `this + 4` into it, scales them by the float at `info + 0x10`
/// (`FLD`/`FSTP` copy; [`hk_ball_and_socket_constraint_cinfo_apply_scale`] unless it equals
/// 1.0) and answers what `hkConstraintCinfo::CloneData` (`00c8a1b0`) answers on
/// `(this, data, info)`.
pub fn hk_ball_and_socket_constraint_cinfo_clone_data(
    e: &mut Engine,
    this: Ptr,
    data: Ptr,
    info: Ptr,
) -> u32 {
    let (word, saved) = enter_scope(e);
    let source = e.mem.u32(this.addr() + 4);
    let destination = if data.addr() == 0 {
        new_constraint_data(e)
    } else {
        data.addr()
    };
    for offset in (0x20..0x40u32).step_by(4) {
        let value = e.mem.u32(source + offset);
        e.mem.set_u32(destination + offset, value);
    }
    let scale = x87_quiet(e.mem.f32(info.addr() + 0x10));
    if scale != 1.0 {
        hk_ball_and_socket_constraint_cinfo_apply_scale(e, scale.to_bits(), Ptr::new(destination));
    }
    let answer = e
        .call(CONSTRAINT_CINFO_CLONE_DATA, &args![this, destination, info])
        .u32();
    e.mem.set_u32(word, saved);
    answer
}

// Translated from 00cc9890 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc9890 - the FS:[0] exception-unwinding frame is not modeled.
/// `hkBallAndSocketConstraintCinfo::CreateData` (Xbox PDB) with the allocation scope word set
/// to 0x10: when `this + 4` is 0, builds a 0x50-byte `hkpBallAndSocketConstraintData`
/// (`00d54290`) there. Answers 0x50.
pub fn hk_ball_and_socket_constraint_cinfo_create_data(e: &mut Engine, this: Ptr) -> u32 {
    let (word, saved) = enter_scope(e);
    if e.mem.u32(this.addr() + 4) == 0 {
        let data = new_constraint_data(e);
        e.mem.set_u32(this.addr() + 4, data);
    }
    e.mem.set_u32(word, saved);
    0x50
}

// Translated from 00cc9940 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc9940 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkBallAndSocketConstraint::CreateClone` (Xbox PDB) with the allocation scope word set to
/// 0x10 and the clone lock held (`00c876f0`, `00c87700`): builds a new
/// `bhkBallAndSocketConstraint` and runs `bhkConstraint::CopyMembers` (`00c89730`) on
/// `(this, clone, process)` (with 0 when the allocation failed). Answers the clone.
pub fn bhk_ball_and_socket_constraint_create_clone(e: &mut Engine, this: Ptr, process: u32) -> u32 {
    let (word, saved) = enter_scope(e);
    e.call(CLONE_LOCK, &args![]);
    let clone = construct_constraint(e);
    e.call(CONSTRAINT_COPY_MEMBERS, &args![this, clone, process]);
    e.call(CLONE_UNLOCK, &args![]);
    e.mem.set_u32(word, saved);
    clone
}

// Translated from 00cc9a20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc9a20 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkBallAndSocketConstraint::CreateCinfo` (Xbox PDB), with the allocation scope word set
/// to 0x10: stores in the byte at `created` whether the cinfo at `+0xC` was missing; then
/// builds it: a 0x14-byte block from the memory manager (`00aa3e40`) whose `+8` is 1, `+0xC`
/// and `+0x10` are 0, vtable is `010c8cc8` and whose `+4` is 0, the cinfo being the address
/// `block + 4` (0 when the manager answers 0), stored at `+0xC` and, when `+8` is not 0,
/// filled by `bhkConstraint::CopyCinfo` (`00c89c80`) given the block (0 for no cinfo).
/// Answers the cinfo.
pub fn bhk_ball_and_socket_constraint_create_cinfo(e: &mut Engine, this: Ptr, created: Ptr) -> u32 {
    let (word, saved) = enter_scope(e);
    let missing = e.mem.u32(this.addr() + 0xc) == 0;
    e.mem.set_u8(created.addr(), u8::from(missing));
    if missing {
        let block = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x14u32])
            .u32();
        let cinfo = if block == 0 {
            0
        } else {
            e.mem.set_u32(block + 8, 1);
            e.mem.set_u32(block + 0xc, 0);
            e.mem.set_u32(block + 0x10, 0);
            e.mem.set_u32(block, VTABLE_CINFO_BLOCK);
            e.mem.set_u32(block + 4, 0);
            block + 4
        };
        e.mem.set_u32(this.addr() + 0xc, cinfo);
        if e.mem.u32(this.addr() + 8) != 0 {
            let argument = if cinfo == 0 { 0 } else { cinfo - 4 };
            e.call(CONSTRAINT_COPY_CINFO, &args![this, argument]);
        }
    }
    let cinfo = e.mem.u32(this.addr() + 0xc);
    e.mem.set_u32(word, saved);
    cinfo
}

// Translated from 00cc9af0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc9af0 - the FS:[0] exception-unwinding frame is not modeled.
/// Unnamed (virtual slot 0xA4, `(this, flag)` with a byte flag), with the allocation scope
/// word set to 0x10: when the low byte of `flag` is not 0, runs virtual slot 0 (the scalar
/// deleting destructor, argument 1) of the cinfo block (`[this + 0xC] - 4`, unless the
/// cinfo is 0) and clears `+0xC`.
pub fn fn_00cc9af0(e: &mut Engine, this: Ptr, flag: u32) {
    let (word, saved) = enter_scope(e);
    if flag as u8 != 0 {
        let cinfo = e.mem.u32(this.addr() + 0xc);
        if cinfo != 0 && cinfo != 4 {
            e.vcall(cinfo - 4, 0, &args![1u32]);
        }
        e.mem.set_u32(this.addr() + 0xc, 0);
    }
    e.mem.set_u32(word, saved);
}

// Translated from 00cc9b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBallAndSocketConstraintCinfo::LoadBinary` (Xbox PDB): `hkConstraintCinfo::LoadBinary`
/// (`00c8a230`) on `(this, stream)`, then reads the two vectors of 16 bytes at `+0x20` and
/// `+0x30` of the data block at `this + 4` through the read function of the stream's file
/// (`[stream + 0x24C]`, function at `+8`, `(file, destination, 0x10, &0x10, 1)`), adding
/// each count to the file position at `+4`.
pub fn hk_ball_and_socket_constraint_cinfo_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(CONSTRAINT_CINFO_LOAD_BINARY, &args![this, stream]);
    let data = e.mem.u32(this.addr() + 4);
    for offset in [0x20u32, 0x30] {
        let file = e.mem.u32(stream.addr() + 0x24c);
        let read = e.mem.u32(file + 8);
        let count = e.with_stack(4, |e, size| {
            e.mem.set_u32(size.addr(), 0x10);
            e.call(read, &args![file, data + offset, 0x10u32, size, 1u32])
                .u32()
        });
        let position = e.mem.u32(file + 4);
        e.mem.set_u32(file + 4, position.wrapping_add(count));
    }
}

// Translated from 00cc9bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBallAndSocketConstraintCinfo::SaveBinary` (Xbox PDB): `hkConstraintCinfo::SaveBinary`
/// (`00c8a280`) on `(this, stream)`, then writes the two vectors of 16 bytes at `+0x20` and
/// `+0x30` of the data block at `this + 4` through the write function of the stream's file
/// (`[stream + 0x250]`, function at `+0xC`, `(file, source, 0x10, &0x10, 1)`), adding each
/// count to the file position at `+4`.
pub fn hk_ball_and_socket_constraint_cinfo_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(CONSTRAINT_CINFO_SAVE_BINARY, &args![this, stream]);
    let data = e.mem.u32(this.addr() + 4);
    for offset in [0x20u32, 0x30] {
        let file = e.mem.u32(stream.addr() + 0x250);
        let write = e.mem.u32(file + 0xc);
        let count = e.with_stack(4, |e, size| {
            e.mem.set_u32(size.addr(), 0x10);
            e.call(write, &args![file, data + offset, 0x10u32, size, 1u32])
                .u32()
        });
        let position = e.mem.u32(file + 4);
        e.mem.set_u32(file + 4, position.wrapping_add(count));
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cc9640, bhk_ball_and_socket_constraint_destructor(Ptr)),
        entry!(
            0x00cc96b0,
            hk_ball_and_socket_constraint_cinfo_apply_scale(u32, Ptr)
        ),
        entry!(0x00cc9700, fn_00cc9700() -> u32),
        entry!(0x00cc9790, hk_ball_and_socket_constraint_cinfo_clone_data(Ptr, Ptr, Ptr) -> u32),
        entry!(0x00cc9890, hk_ball_and_socket_constraint_cinfo_create_data(Ptr) -> u32),
        entry!(0x00cc9940, bhk_ball_and_socket_constraint_create_clone(Ptr, u32) -> u32),
        entry!(0x00cc9a20, bhk_ball_and_socket_constraint_create_cinfo(Ptr, Ptr) -> u32),
        entry!(0x00cc9af0, fn_00cc9af0(Ptr, u32)),
        entry!(
            0x00cc9b80,
            hk_ball_and_socket_constraint_cinfo_load_binary(Ptr, Ptr)
        ),
        entry!(
            0x00cc9bf0,
            hk_ball_and_socket_constraint_cinfo_save_binary(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
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

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn map_counters(e: &mut Engine) {
        for counter in [COUNT_BASE, COUNT_CLASS] {
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

    /// Makes the memory router answer an allocator whose slot 4 returns `block`, recording
    /// the requested size in the word at `block + 0x48`.
    fn route_allocation(e: &mut Engine, block: u32) {
        let allocator = object(e, &[(4, 0x0900_0004)]);
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
    }

    #[test]
    fn destructor_stores_the_vtable_kills_and_destroys_the_base() {
        let mut e = Engine::new();
        map_counters(&mut e);
        e.register(KILL, nothing);
        e.register(CONSTRAINT_DESTRUCTOR, nothing);
        let this = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        e.call(0x00cc9640, &args![this]);
        let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(order, [0x00cc9640, KILL, CONSTRAINT_DESTRUCTOR]);
        assert_eq!(e.mem.u32(this), VTABLE_CLASS);
        assert_eq!(e.global::<u32>(COUNT_CLASS), 9);
    }

    #[test]
    fn apply_scale_multiplies_both_vectors_unless_the_scale_is_one() {
        let mut e = Engine::new();
        let cinfo = e.mem.alloc(0x50);
        for lane in 0..8u32 {
            e.mem.set_f32(cinfo + 0x20 + 4 * lane, 1.0 + lane as f32);
        }
        e.mem.set_f32(cinfo + 0x1c, 7.0);
        e.call(0x00cc96b0, &args![1.0f32.to_bits(), cinfo]);
        assert_eq!(e.mem.f32(cinfo + 0x24), 2.0);
        e.call(0x00cc96b0, &args![2.5f32.to_bits(), 0u32]);
        assert_eq!(e.mem.f32(cinfo + 0x24), 2.0);
        e.call(0x00cc96b0, &args![2.5f32.to_bits(), cinfo]);
        for lane in 0..8u32 {
            assert_eq!(
                e.mem.f32(cinfo + 0x20 + 4 * lane),
                2.5 * (1.0 + lane as f32)
            );
        }
        assert_eq!(e.mem.f32(cinfo + 0x1c), 7.0);
        assert_eq!(e.mem.u32(cinfo + 0x40), 0);
        e.call(0x00cc96b0, &args![f32::NAN.to_bits(), cinfo]);
        assert!(e.mem.f32(cinfo + 0x20).is_nan());
        assert!(e.mem.f32(cinfo + 0x3c).is_nan());
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
        assert_eq!(e.call(0x00cc9700, &args![]).u32(), block);
        assert_eq!(calls(&e, HEAP_ALLOCATE), [[0x10]]);
        assert_eq!(calls(&e, REF_OBJECT_CONSTRUCT), [[block]]);
        assert_eq!(e.mem.u32(block), VTABLE_CLASS);
        assert_eq!(e.mem.u32(block + 0xc), 0);
        assert_eq!(e.global::<u32>(COUNT_BASE), 11);
        assert_eq!(e.global::<u32>(COUNT_CLASS), 11);
        e.register(HEAP_ALLOCATE, nothing);
        assert_eq!(e.call(0x00cc9700, &args![]).u32(), 0);
        assert_eq!(e.global::<u32>(COUNT_CLASS), 11);
    }

    /// A wrapper whose data block (at `+4`) holds the vectors 1..=8 at `+0x20`.
    fn wrapper_with_data(e: &mut Engine) -> (u32, u32) {
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(0x50);
        e.mem.set_u32(this + 4, data);
        for lane in 0..8u32 {
            e.mem.set_f32(data + 0x20 + 4 * lane, 1.0 + lane as f32);
        }
        (this, data)
    }

    #[test]
    fn clone_data_copies_and_scales_the_vectors_into_the_given_data() {
        for (scale, factor) in [(1.0f32, 1.0f32), (3.0, 3.0)] {
            let mut e = Engine::new();
            let word = e.tls() + SCOPE_WORD;
            e.mem.set_u32(word, 0x55);
            let (this, _) = wrapper_with_data(&mut e);
            let destination = e.mem.alloc(0x50);
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
            let answer = e.call(0x00cc9790, &args![this, destination, info]).u32();
            assert_eq!(answer, 0xc10e);
            for lane in 0..8u32 {
                assert_eq!(
                    e.mem.f32(destination + 0x20 + 4 * lane),
                    factor * (1.0 + lane as f32)
                );
            }
            assert_eq!(
                calls(&e, CONSTRAINT_CINFO_CLONE_DATA),
                [[this, destination, info]]
            );
            assert_eq!(e.mem.u32(word), 0x55);
        }
    }

    #[test]
    fn clone_data_builds_a_new_data_block_when_none_is_given() {
        let mut e = Engine::new();
        let (this, _) = wrapper_with_data(&mut e);
        let block = e.mem.alloc(0x60);
        route_allocation(&mut e, block);
        e.register(BALL_AND_SOCKET_DATA_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(CONSTRAINT_CINFO_CLONE_DATA, nothing);
        let info = e.mem.alloc(0x20);
        e.mem.set_f32(info + 0x10, 1.0);
        e.call_log = Some(vec![]);
        e.call(0x00cc9790, &args![this, 0u32, info]);
        assert_eq!(e.mem.u32(block + 0x48), 0x50);
        assert_eq!(e.mem.u16(block + 4), 0x50);
        assert_eq!(calls(&e, BALL_AND_SOCKET_DATA_CONSTRUCT), [[block]]);
        assert_eq!(e.mem.f32(block + 0x3c), 8.0);
        assert_eq!(
            calls(&e, CONSTRAINT_CINFO_CLONE_DATA),
            [[this, block, info]]
        );
    }

    #[test]
    fn clone_data_scales_with_a_nan_scale_and_quiets_it() {
        let mut e = Engine::new();
        let (this, _) = wrapper_with_data(&mut e);
        let destination = e.mem.alloc(0x50);
        let info = e.mem.alloc(0x20);
        e.mem.set_u32(info + 0x10, 0x7f80_0001);
        e.register(CONSTRAINT_CINFO_CLONE_DATA, nothing);
        e.call(0x00cc9790, &args![this, destination, info]);
        assert!(e.mem.f32(destination + 0x20).is_nan());
        assert!(e.mem.f32(destination + 0x3c).is_nan());
    }

    #[test]
    fn create_data_builds_the_data_block_once() {
        let mut e = Engine::new();
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 0x55);
        let block = e.mem.alloc(0x60);
        route_allocation(&mut e, block);
        e.register(BALL_AND_SOCKET_DATA_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00cc9890, &args![this]).u32(), 0x50);
        assert_eq!(e.mem.u32(this + 4), block);
        assert_eq!(e.mem.u16(block + 4), 0x50);
        assert_eq!(e.call(0x00cc9890, &args![this]).u32(), 0x50);
        assert_eq!(calls(&e, BALL_AND_SOCKET_DATA_CONSTRUCT).len(), 1);
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
            CONSTRAINT_COPY_MEMBERS,
        ] {
            e.register(address, nothing);
        }
        let this = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00cc9940, &args![this, 0x7700u32]).u32(), block);
        let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            [
                0x00cc9940,
                CLONE_LOCK,
                HEAP_ALLOCATE,
                REF_OBJECT_CONSTRUCT,
                CONSTRAINT_COPY_MEMBERS,
                CLONE_UNLOCK
            ]
        );
        assert_eq!(calls(&e, CONSTRAINT_COPY_MEMBERS), [[this, block, 0x7700]]);
        assert_eq!(e.mem.u32(block), VTABLE_CLASS);
        assert_eq!(e.mem.u32(word), 0x55);
        // A failed allocation still runs the copy, with 0.
        e.register(HEAP_ALLOCATE, nothing);
        assert_eq!(e.call(0x00cc9940, &args![this, 0x7700u32]).u32(), 0);
        assert_eq!(calls(&e, CONSTRAINT_COPY_MEMBERS)[1], [this, 0, 0x7700]);
    }

    #[test]
    fn create_cinfo_builds_the_block_once_and_fills_it_when_the_havok_object_exists() {
        let mut e = Engine::new();
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 0x55);
        let block = e.mem.alloc(0x14);
        e.register_double(MEMORY_ALLOCATE, move |_, _| Ret {
            eax: block,
            ..Ret::default()
        });
        e.register(CONSTRAINT_COPY_CINFO, nothing);
        let this = e.mem.alloc(0x20);
        let created = e.mem.alloc(4);
        e.mem.set_u32(this + 8, 0x1234);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00cc9a20, &args![this, created]).u32(), block + 4);
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(calls(&e, MEMORY_ALLOCATE), [[MEMORY_MANAGER, 0x14]]);
        assert_eq!(e.mem.u32(block), VTABLE_CINFO_BLOCK);
        assert_eq!(e.mem.u32(block + 8), 1);
        assert_eq!(e.mem.u32(block + 0x10), 0);
        assert_eq!(e.mem.u32(this + 0xc), block + 4);
        assert_eq!(calls(&e, CONSTRAINT_COPY_CINFO), [[this, block]]);
        // Second time: nothing is built.
        assert_eq!(e.call(0x00cc9a20, &args![this, created]).u32(), block + 4);
        assert_eq!(e.mem.u8(created), 0);
        assert_eq!(calls(&e, MEMORY_ALLOCATE).len(), 1);
        assert_eq!(e.mem.u32(word), 0x55);
        // No Havok object: no copy; a failed allocation leaves 0.
        let bare = e.mem.alloc(0x20);
        e.register(MEMORY_ALLOCATE, nothing);
        assert_eq!(e.call(0x00cc9a20, &args![bare, created]).u32(), 0);
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(calls(&e, CONSTRAINT_COPY_CINFO).len(), 1);
        e.mem.set_u32(bare + 8, 1);
        e.mem.set_u32(bare + 0xc, 0);
        e.call(0x00cc9a20, &args![bare, created]);
        assert_eq!(calls(&e, CONSTRAINT_COPY_CINFO)[1], [bare, 0]);
    }

    #[test]
    fn fn_00cc9af0_destroys_the_cinfo_block_only_for_a_nonzero_flag() {
        let mut e = Engine::new();
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 0x55);
        e.register_double(0x0900_0000, |e, _| {
            let scope = e.tls() + SCOPE_WORD;
            assert_eq!(e.mem.u32(scope), 0x10);
            Ret::default()
        });
        let block = object(&mut e, &[(0, 0x0900_0000)]);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0xc, block + 4);
        e.call_log = Some(vec![]);
        e.call(0x00cc9af0, &args![this, 0x100u32]);
        assert_eq!(e.mem.u32(this + 0xc), block + 4);
        assert!(calls(&e, 0x0900_0000).is_empty());
        e.call(0x00cc9af0, &args![this, 0x101u32]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(calls(&e, 0x0900_0000), [[block, 1]]);
        // Nothing to destroy.
        e.call(0x00cc9af0, &args![this, 1u32]);
        assert_eq!(calls(&e, 0x0900_0000).len(), 1);
        assert_eq!(e.mem.u32(word), 0x55);
    }

    /// A wrapper with a data block at `+4` and a stream whose file (read at `+0x24C`, write
    /// at `+0x250`) has the functions 0x0900_0040 (read) and 0x0900_0044 (write).
    fn stream_setup(e: &mut Engine) -> (u32, u32, u32, u32) {
        let (this, data) = wrapper_with_data(e);
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
    fn load_binary_reads_the_two_vectors_after_the_base() {
        let mut e = Engine::new();
        let (this, data, stream, file) = stream_setup(&mut e);
        e.register(CONSTRAINT_CINFO_LOAD_BINARY, nothing);
        e.register(0x0900_0040, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (0x10, 0x10, 1));
            e.mem.set_u32(a[1], 0xabcd);
            Ret {
                eax: 0x10,
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        e.call(0x00cc9b80, &args![this, stream]);
        assert_eq!(calls(&e, CONSTRAINT_CINFO_LOAD_BINARY), [[this, stream]]);
        let reads: Vec<(u32, u32)> = calls(&e, 0x0900_0040)
            .iter()
            .map(|c| (c[0], c[1]))
            .collect();
        assert_eq!(reads, [(file, data + 0x20), (file, data + 0x30)]);
        assert_eq!(e.mem.u32(data + 0x20), 0xabcd);
        assert_eq!(e.mem.u32(data + 0x30), 0xabcd);
        assert_eq!(e.mem.u32(file + 4), 132);
    }

    #[test]
    fn save_binary_writes_the_two_vectors_after_the_base() {
        let mut e = Engine::new();
        let (this, data, stream, file) = stream_setup(&mut e);
        e.register(CONSTRAINT_CINFO_SAVE_BINARY, nothing);
        e.register(0x0900_0044, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (0x10, 0x10, 1));
            Ret {
                eax: 0x10,
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        e.call(0x00cc9bf0, &args![this, stream]);
        assert_eq!(calls(&e, CONSTRAINT_CINFO_SAVE_BINARY), [[this, stream]]);
        let writes: Vec<(u32, u32)> = calls(&e, 0x0900_0044)
            .iter()
            .map(|c| (c[0], c[1]))
            .collect();
        assert_eq!(writes, [(file, data + 0x20), (file, data + 0x30)]);
        assert_eq!(e.mem.u32(file + 4), 132);
    }
}
