//! `BSHavok/bhkposearray.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A `bhkPoseArray` holds, at `+8` and `+0xC`, a count and an array of fixed strings (the
//! bone names; the array block keeps its count in the word before it) and, at `+0x10` and
//! `+0x14`, a count and an array of pointers to poses. A pose is an 8-byte block: a count
//! and a pointer to that many 0x30-byte transforms.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::bshavok::bhkconstraint::{enter_scope, leave_scope, stream_transfer};

/// The pooled allocator object at `011f6238` and its allocate `(size)` method (`00aa3e40`).
const POOL: u32 = 0x011f_6238;
const POOL_ALLOCATE: u32 = 0x00aa_3e40;
/// `NiObject::LoadBinary` (`00a5d610`, `(this, stream)`, `RET 4`).
const NI_OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;
/// `NiStream::LoadFixedString` (`00a64df0`, `this` = stream, one argument: the string slot).
const LOAD_FIXED_STRING: u32 = 0x00a6_4df0;
/// `bhkStream` function (`00cf0b00`, cdecl `(stream, transform)`) that loads one 0x30-byte
/// transform.
const LOAD_TRANSFORM: u32 = 0x00cf_0b00;
/// `__eh_vector_constructor_iterator` (`00ec782f`: array, element size, count,
/// constructor, destructor) with the fixed string constructor (`007613f0`) and destructor
/// (`004381b0`).
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x00ec_782f;
const STRING_CONSTRUCT: u32 = 0x0076_13f0;
const STRING_DESTRUCT: u32 = 0x0043_81b0;

/// The byte size of an array of `count` 4-byte items, saturated to `0xFFFFFFFF` when the
/// multiplication overflows (the code ORs in an all-ones word on carry).
fn array_bytes(count: u32) -> u32 {
    count.saturating_mul(4)
}

// Translated from 00cc1730 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc1730 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkPoseArray::LoadBinary` (Ghidra name), `(this, stream)`: in the Havok allocation
/// scope loads the base (`NiObject::LoadBinary`), reads the name count into `+8`, allocates
/// the name array from the pool (a count word plus the strings, constructed with
/// `__eh_vector_constructor_iterator`; the size saturates on overflow and a failed
/// allocation leaves `+0xC` null) and loads each name with `NiStream::LoadFixedString`.
/// Then it reads the pose count into `+0x10`, allocates the pose pointer array (no count
/// word, no null check) into `+0x14`, and for each pose allocates an 8-byte block (zeroed;
/// null stays null), stores it, reads its transform count and calls `00cf0b00` for each
/// transform at `[pose + 4] + 0x30 * i` (the transform array itself is never allocated
/// here). Every read goes through the stream reader object at `stream + 0x24c` (function
/// pointer in its slot `+8`).
pub fn bhk_pose_array_load_binary(e: &mut Engine, this: Ptr, stream: u32) {
    let this = this.addr();
    let old = enter_scope(e);
    e.call(NI_OBJECT_LOAD_BINARY, &args![this, stream]);
    let reader = e.mem.u32(stream + 0x24c);
    stream_transfer(e, reader, 8, this + 8, 4);
    let names = e.mem.u32(this + 8);
    if names != 0 {
        let size = array_bytes(names).saturating_add(4);
        let block = e.call(POOL_ALLOCATE, &args![POOL, size]).u32();
        let array = if block == 0 {
            0
        } else {
            e.mem.set_u32(block, names);
            e.call(
                VECTOR_CONSTRUCTOR_ITERATOR,
                &args![block + 4, 4u32, names, STRING_CONSTRUCT, STRING_DESTRUCT],
            );
            block + 4
        };
        e.mem.set_u32(this + 0xc, array);
    }
    let mut index = 0u32;
    while index < e.mem.u32(this + 8) {
        let array = e.mem.u32(this + 0xc);
        e.call(LOAD_FIXED_STRING, &args![stream, array + index * 4]);
        index += 1;
    }
    let reader = e.mem.u32(stream + 0x24c);
    stream_transfer(e, reader, 8, this + 0x10, 4);
    let poses = e.mem.u32(this + 0x10);
    if poses != 0 {
        let array = e
            .call(POOL_ALLOCATE, &args![POOL, array_bytes(poses)])
            .u32();
        e.mem.set_u32(this + 0x14, array);
    }
    let mut index = 0u32;
    while index < e.mem.u32(this + 0x10) {
        let pose = e.call(POOL_ALLOCATE, &args![POOL, 8u32]).u32();
        if pose != 0 {
            e.mem.set_u32(pose, 0);
            e.mem.set_u32(pose + 4, 0);
        }
        let slots = e.mem.u32(this + 0x14);
        e.mem.set_u32(slots + index * 4, pose);
        let pose = e.mem.u32(e.mem.u32(this + 0x14) + index * 4);
        let reader = e.mem.u32(stream + 0x24c);
        stream_transfer(e, reader, 8, pose, 4);
        let mut offset = 0u32;
        let mut done = 0u32;
        while done < e.mem.u32(pose) {
            let transforms = e.mem.u32(pose + 4);
            e.call(
                LOAD_TRANSFORM,
                &args![stream, transforms.wrapping_add(offset)],
            );
            offset = offset.wrapping_add(0x30);
            done += 1;
        }
        index += 1;
    }
    leave_scope(e, old);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00cc1730, bhk_pose_array_load_binary(Ptr, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Calls = Vec<Vec<u32>>;

    fn calls_to(e: &Engine, addr: u32) -> Calls {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|call| call.0 == addr)
            .map(|call| call.1.clone())
            .collect()
    }

    /// A stream whose reader (at `+0x24c`) fills each destination from a queue of words
    /// kept in the reader block (`reader + 0x10`: next index; the words from `+0x20`).
    fn stream_with(e: &mut Engine, words: &[u32]) -> u32 {
        let stream = e.mem.alloc(0x300);
        let reader = e.mem.alloc(0x200);
        e.mem.set_u32(stream + 0x24c, reader);
        e.mem.set_u32(reader + 8, 0x0900_0001);
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(reader + 0x20 + 4 * i as u32, *word);
        }
        e.register(0x0900_0001, |e, a| {
            let next = e.mem.u32(a[0] + 0x10);
            let word = e.mem.u32(a[0] + 0x20 + 4 * next);
            e.mem.set_u32(a[1], word);
            e.mem.set_u32(a[0] + 0x10, next + 1);
            Ret {
                eax: 4,
                ..Ret::default()
            }
        });
        stream
    }

    fn quiet_callees(e: &mut Engine) {
        e.register(NI_OBJECT_LOAD_BINARY, |_, _| Ret::default());
        e.register(VECTOR_CONSTRUCTOR_ITERATOR, |_, _| Ret::default());
        e.register(LOAD_FIXED_STRING, |_, _| Ret::default());
        e.register(LOAD_TRANSFORM, |_, _| Ret::default());
    }

    #[test]
    fn array_bytes_saturates_on_overflow() {
        assert_eq!(array_bytes(3), 12);
        assert_eq!(array_bytes(0x3fff_ffff), 0xffff_fffc);
        assert_eq!(array_bytes(0x4000_0000), u32::MAX);
    }

    #[test]
    fn bhk_pose_array_load_binary_reads_names_and_poses() {
        let mut e = Engine::new();
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x77);
        // Two names; two poses with 2 and 0 transforms.
        let stream = stream_with(&mut e, &[2, 2, 2, 0]);
        let this = e.mem.alloc(0x20);
        quiet_callees(&mut e);
        e.register(POOL_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00cc1730, &args![this, stream]);
        assert_eq!(
            calls_to(&e, NI_OBJECT_LOAD_BINARY),
            vec![vec![this, stream]]
        );
        assert_eq!(e.mem.u32(this + 8), 2);
        let names = e.mem.u32(this + 0xc);
        assert_eq!(e.mem.u32(names - 4), 2);
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCTOR_ITERATOR),
            vec![vec![names, 4, 2, STRING_CONSTRUCT, STRING_DESTRUCT]]
        );
        assert_eq!(
            calls_to(&e, LOAD_FIXED_STRING),
            vec![vec![stream, names], vec![stream, names + 4]]
        );
        assert_eq!(e.mem.u32(this + 0x10), 2);
        assert_eq!(
            calls_to(&e, POOL_ALLOCATE),
            vec![vec![POOL, 12], vec![POOL, 8], vec![POOL, 8], vec![POOL, 8]]
        );
        let poses = e.mem.u32(this + 0x14);
        let first = e.mem.u32(poses);
        let second = e.mem.u32(poses + 4);
        assert_eq!(e.mem.u32(first), 2);
        assert_eq!(e.mem.u32(second), 0);
        // The transform array is never allocated, so the loads start at null.
        assert_eq!(
            calls_to(&e, LOAD_TRANSFORM),
            vec![vec![stream, 0], vec![stream, 0x30]]
        );
        assert_eq!(e.mem.u32(word), 0x77);
    }

    #[test]
    fn bhk_pose_array_load_binary_with_nothing_to_load_allocates_nothing() {
        let mut e = Engine::new();
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x55);
        let stream = stream_with(&mut e, &[0, 0]);
        let this = e.mem.alloc(0x20);
        quiet_callees(&mut e);
        e.register(POOL_ALLOCATE, |_, _| panic!("no allocation expected"));
        e.call(0x00cc1730, &args![this, stream]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(e.mem.u32(word), 0x55);
    }

    #[test]
    fn bhk_pose_array_load_binary_keeps_a_failed_name_block_null() {
        let mut e = Engine::new();
        // One name whose block cannot be allocated; no poses.
        let stream = stream_with(&mut e, &[1, 0]);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0xc, 0x1234);
        quiet_callees(&mut e);
        e.register(POOL_ALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00cc1730, &args![this, stream]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert!(calls_to(&e, VECTOR_CONSTRUCTOR_ITERATOR).is_empty());
        assert_eq!(calls_to(&e, LOAD_FIXED_STRING), vec![vec![stream, 0]]);
    }
}
