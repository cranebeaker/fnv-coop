//! `fallout/ai/run_009062e0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The lock at `this` +0x20: `0040fbf0` takes it (`thiscall`, one word, 0 here) and
/// `0040fba0` releases it (`thiscall`, no stack words); no engine-map names.
const LOCK: u32 = 0x0040_fbf0;
const UNLOCK: u32 = 0x0040_fba0;
const LOCK_OFFSET: u32 = 0x20;
/// `00449f80` (`thiscall`, no stack words; another unit): answers the map's inner object
/// (`0044d5c0` of the object at +0x14).
const INNER: u32 = 0x0044_9f80;
/// `00528bb0` (`thiscall`, one word; unit `fallout shared/exteriorcellloader.cpp`).
const INNER_TEST: u32 = 0x0052_8bb0;
/// The scope `00404eb0` builds on a stack local (`thiscall`, four words) and `00404ee0`
/// ends (`thiscall`); unit `fallout shared/magic/effectitem.cpp` in the engine map.
const SCOPE_BEGIN: u32 = 0x0040_4eb0;
const SCOPE_END: u32 = 0x0040_4ee0;
/// The allocation and free used here (`cdecl`, one word).
const ALLOCATE: u32 = 0x0040_1000;
const FREE: u32 = 0x0040_1030;
/// `0044d000` (`thiscall`, four words; another unit): builds the 0x24-byte iterator.
const ITERATOR_CONSTRUCT: u32 = 0x0044_d000;
/// The `LockFreeMap<MobileObject *, NiPointer<...>>` vtable (RTTI) `fn_009064e0` stores.
const LOCK_FREE_MAP_VTABLE: u32 = 0x0108_864c;
/// `00528e40` (`thiscall`, one word) and `0044b4b0` (`thiscall`; unit
/// `fallout shared/modelloader.cpp`): run by the destructor before and after the frees.
const MAP_CLEAR: u32 = 0x0052_8e40;
const BASE_DESTRUCT: u32 = 0x0044_b4b0;
/// The iterator's calls (`thiscall`): `006ebde0` (unit
/// `fallout shared/pathfinding/pathmanager.cpp`) answers whether it is finished, `0044de20`
/// whether it has started, `0044de40` sets its started byte (one word) and `0044de80` its
/// finished byte (one word); the engine map's names for the last two (`MapMarkerData`
/// setters) are folded code.
const ITERATOR_DONE: u32 = 0x006e_bde0;
const ITERATOR_STARTED: u32 = 0x0044_de20;
const ITERATOR_SET_STARTED: u32 = 0x0044_de40;
const ITERATOR_SET_FINISHED: u32 = 0x0044_de80;
/// `0044ddc0` (`thiscall` on the map, no stack words): the bucket count.
const BUCKET_COUNT: u32 = 0x0044_ddc0;
/// The inner object's first and next lookups (`thiscall`, three words: bucket, address of
/// the key slot, wanted).
const INNER_FIRST: u32 = 0x0090_6af0;
const INNER_NEXT: u32 = 0x0090_6c10;

// Translated from 009062e0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word, `RET 4`; a `LockFreeMap` method): with
/// the lock at +0x20 held, answers `00528bb0(key)` of the inner object (`00449f80`; the
/// pushed key belongs to `00528bb0`, `00449f80` takes none).
pub fn fn_009062e0(e: &mut Engine, this: u32, key: u32) -> bool {
    e.call(LOCK, &args![this + LOCK_OFFSET, 0u32]);
    let inner = e.call(INNER, &args![this]).u32();
    let result = e.call(INNER_TEST, &args![inner, key]).bool();
    e.call(UNLOCK, &args![this + LOCK_OFFSET]);
    result
}

// Translated from 00906400 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word, `RET 4`; a `LockFreeMap` method): inside
/// a scope local (`00404eb0(6, 1, 010171a0, 0x18e)`, ended by `00404ee0`), a new 0x24-byte
/// iterator (`00401000`) built by `0044d000(this, &bucket[0], &bucket[1], &bucket[2])` for
/// the 12-byte bucket `index` of the array at +0x4; answers it, or 0 when the allocation
/// failed.
// Translation incomplete: 00906400 - the FS:[0] exception frame (ending the scope and freeing the block on unwinding) is not translated
pub fn fn_00906400(e: &mut Engine, this: u32, index: u32) -> u32 {
    e.with_stack(4, |e, scope| {
        let scope = scope.addr();
        e.call(
            SCOPE_BEGIN,
            &args![scope, 6u32, 1u32, 0x0101_71a0u32, 0x18eu32],
        );
        let block = e.call(ALLOCATE, &args![0x24u32]).u32();
        let iterator = if block != 0 {
            let bucket = e
                .mem
                .u32(this + 4)
                .wrapping_add(index.wrapping_mul(3).wrapping_mul(4));
            e.call(
                ITERATOR_CONSTRUCT,
                &args![
                    block,
                    this,
                    bucket,
                    bucket.wrapping_add(4),
                    bucket.wrapping_add(8)
                ],
            )
            .u32()
        } else {
            0
        };
        e.call(SCOPE_END, &args![scope]);
        iterator
    })
}

// Translated from 009064e0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; by the vtable it stores the
/// `LockFreeMap<MobileObject *, NiPointer<...>>` destructor (RTTI)): stores the vtable,
/// runs `00528e40(1)`, frees the blocks at +0xC and +0x4 (`00401030`) and runs `0044b4b0`.
// Translation incomplete: 009064e0 - the FS:[0] exception frame (running 0044b4b0 on unwinding) is not translated
pub fn fn_009064e0(e: &mut Engine, this: u32) {
    e.mem.set_u32(this, LOCK_FREE_MAP_VTABLE);
    e.call(MAP_CLEAR, &args![this, 1u32]);
    let first = e.mem.u32(this + 0xc);
    e.call(FREE, &args![first]);
    let second = e.mem.u32(this + 4);
    e.call(FREE, &args![second]);
    e.call(BASE_DESTRUCT, &args![this]);
}

// Translated from 00906570 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, four words, `RET 0x10`; a `LockFreeMap` method
/// stepping an iterator): with the lock at +0x20 held, false at once when the iterator is
/// finished (`006ebde0`). Otherwise, in a loop (its `MOV ECX,1; TEST ECX,ECX` condition is
/// always true): the inner object's first lookup (`00906af0`, then the started byte set) or,
/// once started, its next lookup (`00906c10`), with the iterator's bucket (+0x4), the
/// address of its key slot (+0x8) and `wanted`. A hit stores the key slot's word at `out`
/// and answers true. A miss moves to the next bucket; past the last bucket (`0044ddc0`) the
/// finished byte is set and false answered; else when `walk_on` is set the started byte
/// is cleared and the loop goes on, otherwise false.
pub fn fn_00906570(
    e: &mut Engine,
    this: u32,
    iterator: u32,
    out: u32,
    wanted: u32,
    walk_on: u32,
) -> bool {
    e.call(LOCK, &args![this + LOCK_OFFSET, 0u32]);
    let mut found = false;
    if !e.call(ITERATOR_DONE, &args![iterator]).bool() {
        loop {
            if !e.call(ITERATOR_STARTED, &args![iterator]).bool() {
                let bucket = e.mem.u32(iterator + 4);
                let inner = e.call(INNER, &args![this]).u32();
                found = e
                    .call(INNER_FIRST, &args![inner, bucket, iterator + 8, wanted])
                    .bool();
                e.call(ITERATOR_SET_STARTED, &args![iterator, 1u32]);
            } else {
                let bucket = e.mem.u32(iterator + 4);
                let inner = e.call(INNER, &args![this]).u32();
                found = e
                    .call(INNER_NEXT, &args![inner, bucket, iterator + 8, wanted])
                    .bool();
            }
            if found {
                let key = e.mem.u32(iterator + 8);
                e.mem.set_u32(out, key);
                break;
            }
            let next = e.mem.u32(iterator + 4).wrapping_add(1);
            e.mem.set_u32(iterator + 4, next);
            let count = e.call(BUCKET_COUNT, &args![this]).u32();
            if e.mem.u32(iterator + 4) >= count {
                e.call(ITERATOR_SET_FINISHED, &args![iterator, 1u32]);
                break;
            }
            if walk_on as u8 == 0 {
                break;
            }
            e.call(ITERATOR_SET_STARTED, &args![iterator, 0u32]);
        }
    }
    e.call(UNLOCK, &args![this + LOCK_OFFSET]);
    found
}

/// `0044cbf0` (`thiscall`): the destructor of the `NiPointer` passed by value
/// (it releases what the pointer holds).
const POINTER_RELEASE: u32 = 0x0044_cbf0;

// Translated from 00906320 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00906320 - the compiler's exception frame (FS:[0]) that releases the by-value pointer when a step throws is not translated
/// A `LockFreeMap` method (`thiscall`, two words, `RET 8`): with the lock at
/// +0x20 held, `009066d0(key, &value)` on the inner object (`00449f80`);
/// `value` is an `NiPointer` passed by value, released (`0044cbf0`) on the
/// way out. Answers `009066d0`'s byte.
pub fn fn_00906320(e: &mut Engine, this: u32, key: u32, value: u32) -> u8 {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), value);
        e.call(LOCK, &args![this + LOCK_OFFSET, 0u32]);
        let inner = e.call(INNER, &args![this]).u32();
        let answer = e.call(0x0090_66d0, &args![inner, key, slot]).u8();
        e.call(UNLOCK, &args![this + LOCK_OFFSET]);
        e.call(POINTER_RELEASE, &args![slot]);
        answer
    })
}

// Translated from 009063b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `LockFreeMap` method (`thiscall`, four words, `RET 0x10`): with the
/// lock at +0x20 held, answers `0044d070(first, second, third, flag)` of the
/// inner object (`00449f80`), `flag` passed as its low byte.
pub fn fn_009063b0(
    e: &mut Engine,
    this: u32,
    first: u32,
    second: u32,
    third: u32,
    flag: u32,
) -> u8 {
    e.call(LOCK, &args![this + LOCK_OFFSET, 0u32]);
    let inner = e.call(INNER, &args![this]).u32();
    let answer = e
        .call(
            0x0044_d070,
            &args![inner, first, second, third, flag & 0xff],
        )
        .u8();
    e.call(UNLOCK, &args![this + LOCK_OFFSET]);
    answer
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0090_6320, fn_00906320(u32, u32, u32) -> u8),
        entry!(0x0090_63b0, fn_009063b0(u32, u32, u32, u32, u32) -> u8),
        entry!(0x0090_62e0, fn_009062e0(u32, u32) -> bool),
        entry!(0x0090_6400, fn_00906400(u32, u32) -> u32),
        entry!(0x0090_64e0, fn_009064e0(u32)),
        entry!(0x0090_6570, fn_00906570(u32, u32, u32, u32, u32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn locks(e: &mut Engine) {
        e.register(LOCK, |_, _| Ret::default());
        e.register(UNLOCK, |_, _| Ret::default());
        e.register(INNER, |_, a| (a[0] + 0x1000).into_ret());
    }

    #[test]
    fn fn_009062e0_tests_under_the_lock() {
        let mut e = Engine::new();
        locks(&mut e);
        e.register(INNER_TEST, |_, a| u32::from(a[1] == 7).into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x0090_62e0, &[0x4000, 7]).bool());
        assert!(!e.call(0x0090_62e0, &[0x4000, 8]).bool());
        assert_eq!(calls_to(&e, INNER_TEST)[0], vec![0x5000, 7]);
        assert_eq!(calls_to(&e, LOCK)[0], vec![0x4020, 0]);
        assert_eq!(calls_to(&e, UNLOCK).len(), 2);
    }

    #[test]
    fn fn_00906400_builds_an_iterator_for_the_bucket() {
        let mut e = Engine::new();
        e.register(SCOPE_BEGIN, |_, _| Ret::default());
        e.register(SCOPE_END, |_, _| Ret::default());
        e.register(ALLOCATE, |_, _| 0x7000u32.into_ret());
        e.register(ITERATOR_CONSTRUCT, |_, a| a[0].into_ret());
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0x9000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0090_6400, &[this, 2]).u32(), 0x7000);
        assert_eq!(
            calls_to(&e, ITERATOR_CONSTRUCT),
            vec![vec![0x7000, this, 0x9018, 0x901c, 0x9020]]
        );
        let scope = calls_to(&e, SCOPE_BEGIN);
        assert_eq!(&scope[0][1..], &[6, 1, 0x0101_71a0, 0x18e]);
        assert_eq!(calls_to(&e, SCOPE_END), vec![vec![scope[0][0]]]);
        e.register(ALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0090_6400, &[this, 2]).u32(), 0);
        assert!(calls_to(&e, ITERATOR_CONSTRUCT).is_empty());
        assert_eq!(calls_to(&e, SCOPE_END).len(), 1);
    }

    #[test]
    fn fn_009064e0_frees_both_blocks() {
        let mut e = Engine::new();
        e.register(MAP_CLEAR, |_, _| Ret::default());
        e.register(FREE, |_, _| Ret::default());
        e.register(BASE_DESTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x24);
        e.mem.set_u32(this + 4, 0x1111);
        e.mem.set_u32(this + 0xc, 0x2222);
        e.call_log = Some(vec![]);
        e.call(0x0090_64e0, &[this]);
        let order: Vec<(u32, Vec<u32>)> = e.call_log.take().unwrap().into_iter().skip(1).collect();
        assert_eq!(
            order,
            vec![
                (MAP_CLEAR, vec![this, 1]),
                (FREE, vec![0x2222]),
                (FREE, vec![0x1111]),
                (BASE_DESTRUCT, vec![this]),
            ]
        );
        assert_eq!(e.mem.u32(this), LOCK_FREE_MAP_VTABLE);
    }

    /// An iterator: finished byte at +0x0, started byte at +0x1, bucket +0x4, key slot +0x8.
    fn iterator_doubles(e: &mut Engine) {
        e.register(ITERATOR_DONE, |e, a| u32::from(e.mem.u8(a[0])).into_ret());
        e.register(ITERATOR_STARTED, |e, a| {
            u32::from(e.mem.u8(a[0] + 1)).into_ret()
        });
        e.register(ITERATOR_SET_STARTED, |e, a| {
            e.mem.set_u8(a[0] + 1, a[1] as u8);
            Ret::default()
        });
        e.register(ITERATOR_SET_FINISHED, |e, a| {
            e.mem.set_u8(a[0], a[1] as u8);
            Ret::default()
        });
        e.register(BUCKET_COUNT, |_, _| 3u32.into_ret());
    }

    #[test]
    fn fn_00906570_walks_the_buckets() {
        let mut e = Engine::new();
        locks(&mut e);
        iterator_doubles(&mut e);
        // The first lookup misses, the next one hits only in bucket 2.
        e.register(INNER_FIRST, |_, _| Ret::default());
        e.register(INNER_NEXT, |e, a| {
            if a[1] == 2 {
                e.mem.set_u32(a[2], 0xabcd);
                1u32.into_ret()
            } else {
                Ret::default()
            }
        });
        let iterator = e.mem.alloc(0xc);
        let out = e.mem.alloc(4);
        // Without walking on: one miss and false.
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0090_6570, &[0x4000, iterator, out, 9, 0]).bool());
        assert_eq!(e.mem.u32(iterator + 4), 1);
        assert_eq!(
            calls_to(&e, INNER_FIRST),
            vec![vec![0x5000, 0, iterator + 8, 9]]
        );
        // Walking on: the started byte cleared again, so bucket 1 is a first lookup too.
        e.mem.set_u8(iterator + 1, 1);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0090_6570, &[0x4000, iterator, out, 9, 1]).bool());
        // Bucket 1 (next lookup, miss), bucket 2 (first lookup after the clear, miss), then
        // past the last bucket: finished.
        assert_eq!(e.mem.u8(iterator), 1);
        assert_eq!(calls_to(&e, INNER_NEXT).len(), 1);
        assert_eq!(calls_to(&e, UNLOCK).len(), 1);
        // Finished: false without a lookup.
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0090_6570, &[0x4000, iterator, out, 9, 1]).bool());
        assert!(calls_to(&e, INNER_FIRST).is_empty());
        // A hit stores the key.
        e.mem.set_u8(iterator, 0);
        e.mem.set_u8(iterator + 1, 1);
        e.mem.set_u32(iterator + 4, 2);
        assert!(e.call(0x0090_6570, &[0x4000, iterator, out, 9, 0]).bool());
        assert_eq!(e.mem.u32(out), 0xabcd);
    }
}

#[cfg(test)]
mod n028_tests {
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
    fn fn_00906320_removes_under_the_lock_and_releases_the_pointer() {
        let mut e = Engine::new();
        for callee in [LOCK, UNLOCK, POINTER_RELEASE] {
            e.register(callee, |_, _| Ret::default());
        }
        e.register(INNER, |_, _| Ret {
            eax: 0x1200,
            ..Ret::default()
        });
        let seen = e.mem.alloc(4);
        e.register_double(0x0090_66d0, move |e, a| {
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(seen, value);
            Ret {
                eax: 0x0101,
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0090_6320, &[0x9000, 7, 0x5e5e]).u8(), 1);
        assert_eq!(e.mem.u32(seen), 0x5e5e);
        assert_eq!(calls_to(&e, LOCK), vec![vec![0x9020, 0]]);
        let call = calls_to(&e, 0x0090_66d0)[0].clone();
        assert_eq!(call[..2], [0x1200, 7]);
        assert_eq!(calls_to(&e, UNLOCK), vec![vec![0x9020]]);
        assert_eq!(calls_to(&e, POINTER_RELEASE), vec![vec![call[2]]]);
    }

    #[test]
    fn fn_009063b0_forwards_under_the_lock() {
        let mut e = Engine::new();
        for callee in [LOCK, UNLOCK] {
            e.register(callee, |_, _| Ret::default());
        }
        e.register(INNER, |_, _| Ret {
            eax: 0x1200,
            ..Ret::default()
        });
        e.register(0x0044_d070, |_, a| Ret {
            eax: a[4],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0090_63b0, &[0x9000, 1, 2, 3, 0x0103]).u8(), 3);
        assert_eq!(calls_to(&e, 0x0044_d070), vec![vec![0x1200, 1, 2, 3, 3]]);
        assert_eq!(calls_to(&e, UNLOCK), vec![vec![0x9020]]);
    }
}
