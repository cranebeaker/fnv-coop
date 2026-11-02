//! `(unplaced)/run_0044c000` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Methods of two `LockFreeMap` instances (`bsmain/ThreadSafeStructures.inl`
//! in the debug strings): `LockFreeMap<const char *, Model *>` (vtable
//! `0x010175bc`) and `LockFreeMap<const char *, KFModel *>` (vtable
//! `0x0101760c`); the class names are the RTTI names their vtables' complete
//! object locators (`0x01106640`, `0x01106654`) give. The map's fields are
//! read at their PC offsets: +0x04 the node array, +0x08 the bucket count,
//! +0x0c the bucket array (a word per bucket, bit 0 a mark), +0x10 the item
//! size word, +0x14 the 16-byte storage object `006c73b0` builds, +0x18 a
//! word cleared with it, +0x20 the lock `0040fbf0` / `0040fba0` take and
//! release.
//!
//! The constructors, destructors and `0044c480` run inside a C++ exception
//! frame (`FS:[0]`) whose unwinding is not translated.

#[allow(unused_imports)]
use crate::prelude::*;

/// `LockFreeMap<const char *, Model *>`'s vtable.
const MODEL_MAP_VTABLE: u32 = 0x0101_75bc;
/// `LockFreeMap<const char *, KFModel *>`'s vtable.
const KF_MODEL_MAP_VTABLE: u32 = 0x0101_760c;
/// `"d:\_fallout3\platforms\common\code\bsmain\ThreadSafeStructures.inl"`.
const THREAD_SAFE_STRUCTURES_FILE: u32 = 0x0101_73f8;

/// The map's insertion step, `thiscall(this, hash, key, value, flag)`
/// (`0044d070`, unnamed in the map).
const MAP_INSERT: u32 = 0x0044_d070;
/// The base constructor of the map (`thiscall(this)`, `0044d7f0`).
const MAP_BASE_CONSTRUCT: u32 = 0x0044_d7f0;
/// The base destructor of the map (`thiscall(this)`, `0044b4b0`).
const MAP_BASE_DESTROY: u32 = 0x0044_b4b0;
/// The lock's constructor, run on `this + 0x20` (`thiscall`, `0096a2d0`).
const LOCK_CONSTRUCT: u32 = 0x0096_a2d0;
/// Takes the lock (`thiscall(lock, 0)`, `0040fbf0`).
const LOCK: u32 = 0x0040_fbf0;
/// Releases the lock (`thiscall(lock)`, `0040fba0`).
const UNLOCK: u32 = 0x0040_fba0;
/// A scope object built on the stack with `(6, 1, file, line)` and closed at
/// the end of the scope (`0040_4eb0` / `00404ee0`, `thiscall`).
const SCOPE_OPEN: u32 = 0x0040_4eb0;
const SCOPE_CLOSE: u32 = 0x0040_4ee0;
/// `operator new(size)` (`cdecl`, `00401000`).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete(block)` (`cdecl`, `00401030`).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The vector constructor iterator `(array, size, count, constructor)`
/// (`stdcall`, `00401050`).
const VECTOR_CONSTRUCT: u32 = 0x0040_1050;
/// The bucket word's constructor handed to [`VECTOR_CONSTRUCT`].
const BUCKET_CONSTRUCT: u32 = 0x0044_dee0;
/// The storage object's constructor `thiscall(block, size)` (`006c73b0`).
const STORAGE_CONSTRUCT: u32 = 0x006c_73b0;
/// The storage object's destructor `thiscall(storage, 1)` (`00666570`).
const STORAGE_DESTROY: u32 = 0x0066_6570;
/// The storage object's first word (`thiscall`, `00559450` returns `[this]`).
const STORAGE_FIRST_WORD: u32 = 0x0055_9450;
/// A marked pointer's value without the mark (`[this] & ~1`, `0044dec0`).
const MARKED_POINTER_GET: u32 = 0x0044_dec0;
/// Clears a marked pointer (`[this] = 0`, `006665a0`).
const MARKED_POINTER_CLEAR: u32 = 0x0066_65a0;
/// Frees a node back to the map (`thiscall(this, node)`, `0044dde0`).
const MAP_FREE_NODE: u32 = 0x0044_dde0;
/// The map object a lookup works on (`thiscall(this)`, `00449f80`).
const MAP_LOOKUP_TARGET: u32 = 0x0044_9f80;
/// The iterator's "at the end" test: bit 1 of the byte at +0x0c
/// (`006ebde0`).
const ITERATOR_AT_END: u32 = 0x006e_bde0;
/// The iterator's bit 0 test (byte at +0x0c, `0044de20`).
const ITERATOR_BIT_0: u32 = 0x0044_de20;
/// Sets (1) or clears (0) bit 0 of the iterator's byte at +0x0c
/// (`0044de40`; the map's name for the folded body is
/// `MapMarkerData::SetVisible`).
const ITERATOR_SET_BIT_0: u32 = 0x0044_de40;
/// Sets (1) or clears (0) bit 1 of the iterator's byte at +0x0c
/// (`0044de80`; the map's name for the folded body is
/// `MapMarkerData::SetTravelLoc`).
const ITERATOR_SET_BIT_1: u32 = 0x0044_de80;
/// The map's bucket count (`[this + 8]`, `0044ddc0`).
const MAP_BUCKET_COUNT: u32 = 0x0044_ddc0;
/// The bucket step taken when the iterator's bit 0 is clear,
/// `thiscall(map, bucket, &key, extra)` (`0044ea60`).
const BUCKET_STEP_FIRST: u32 = 0x0044_ea60;
/// The bucket step taken when the iterator's bit 0 is set (`0044eb80`).
const BUCKET_STEP_NEXT: u32 = 0x0044_eb80;

/// `size * count` as the code computes an allocation size: an overflow
/// gives `0xffffffff` (`MUL`, `SETO`, `NEG`, `OR`).
fn allocation_size(count: u32, size: u32) -> u32 {
    count.saturating_mul(size)
}

// Translated from 0044c000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `thiscall(this, key, value, flag)`: hashes `key` through the object `this`
/// points at (its virtual at +0x24, one argument) and inserts with that hash
/// ([`MAP_INSERT`] on `this`), whose result (`EAX`) it returns.
pub fn fn_0044c000(e: &mut Engine, this: Ptr, key: u32, value: u32, flag: u8) -> u32 {
    let object = e.mem.u32(this.addr());
    let hash = e.vcall(object, 0x24, &args![key]).u32();
    e.call(MAP_INSERT, &args![this, hash, key, value, flag as u32])
        .u32()
}

/// The common body of the two constructors: everything but the vtable.
fn construct(
    e: &mut Engine,
    this: Ptr,
    vtable: u32,
    map_size: u32,
    bucket_count: u32,
    item_size: u32,
) -> Ptr {
    let this_addr = this.addr();
    e.call(MAP_BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(this_addr, vtable);
    e.call(LOCK_CONSTRUCT, &args![this_addr + 0x20]);
    e.with_stack(4, |e, scope| {
        e.call(
            SCOPE_OPEN,
            &args![scope, 6u32, 1u32, THREAD_SAFE_STRUCTURES_FILE, 0x390u32],
        );
        e.mem.set_u32(this_addr + 0x18, 0);
        e.mem.set_u32(this_addr + 8, bucket_count);
        let buckets = e
            .call(OPERATOR_NEW, &args![allocation_size(bucket_count, 4)])
            .u32();
        if buckets != 0 {
            e.call(
                VECTOR_CONSTRUCT,
                &args![buckets, 4u32, bucket_count, BUCKET_CONSTRUCT],
            );
        }
        e.mem.set_u32(this_addr + 0xc, buckets);
        let node_bytes = allocation_size(map_size.wrapping_mul(3), 4);
        let nodes = e.call(OPERATOR_NEW, &args![node_bytes]).u32();
        e.mem.set_u32(this_addr + 4, nodes);
        e.mem.set_u32(this_addr + 0x10, item_size);
        let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
        let storage = if block == 0 {
            0
        } else {
            e.call(STORAGE_CONSTRUCT, &args![block, map_size]).u32()
        };
        e.mem.set_u32(this_addr + 0x14, storage);
        e.call(SCOPE_CLOSE, &args![scope]);
    });
    this
}

// Translation incomplete: 0044c040 - the C++ exception frame (FS:[0]) and its unwind actions are not translated
// Translated from 0044c040 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `LockFreeMap<const char *, Model *>` constructor
/// `thiscall(this, map_size, bucket_count, item_size)`: the base, the
/// vtable, the lock at +0x20, then inside a scope object: the bucket array
/// (`bucket_count` words, each constructed), the node array (`3 * map_size`
/// words), the item size and the storage object for `map_size`. Returns
/// `this`.
pub fn fn_0044c040(
    e: &mut Engine,
    this: Ptr,
    map_size: u32,
    bucket_count: u32,
    item_size: u32,
) -> Ptr {
    construct(e, this, MODEL_MAP_VTABLE, map_size, bucket_count, item_size)
}

// Translated from 0044c1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A key comparison in the `LockFreeMap` vtables (`thiscall(this, a, b)`,
/// `this` unused): whether `a >= b` (unsigned), as 1 or 0.
pub fn fn_0044c1c0(_e: &mut Engine, _this: Ptr, a: u32, b: u32) -> bool {
    a >= b
}

/// The common body of the two destructors: everything but the vtable.
fn destroy(e: &mut Engine, this: Ptr, vtable: u32) {
    e.mem.set_u32(this.addr(), vtable);
    fn_0044c480(e, this, 1);
    let buckets = e.mem.u32(this.addr() + 0xc);
    e.call(OPERATOR_DELETE, &args![buckets]);
    let nodes = e.mem.u32(this.addr() + 4);
    e.call(OPERATOR_DELETE, &args![nodes]);
    e.call(MAP_BASE_DESTROY, &args![this]);
}

// Translation incomplete: 0044c1e0 - the C++ exception frame (FS:[0]) and its unwind actions are not translated
// Translated from 0044c1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `LockFreeMap<const char *, Model *>` destructor (`thiscall(this)`):
/// the vtable, [`fn_0044c480`] with 1 (empties the map and keeps no
/// storage), frees the bucket and node arrays, then the base destructor.
pub fn fn_0044c1e0(e: &mut Engine, this: Ptr) {
    destroy(e, this, MODEL_MAP_VTABLE);
}

// Translation incomplete: 0044c270 - the C++ exception frame (FS:[0]) and its unwind actions are not translated
// Translated from 0044c270 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `LockFreeMap<const char *, KFModel *>` constructor: as
/// [`fn_0044c040`] with its own vtable.
pub fn fn_0044c270(
    e: &mut Engine,
    this: Ptr,
    map_size: u32,
    bucket_count: u32,
    item_size: u32,
) -> Ptr {
    construct(
        e,
        this,
        KF_MODEL_MAP_VTABLE,
        map_size,
        bucket_count,
        item_size,
    )
}

// Translation incomplete: 0044c3f0 - the C++ exception frame (FS:[0]) and its unwind actions are not translated
// Translated from 0044c3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `LockFreeMap<const char *, KFModel *>` destructor: as
/// [`fn_0044c1e0`] with its own vtable.
pub fn fn_0044c3f0(e: &mut Engine, this: Ptr) {
    destroy(e, this, KF_MODEL_MAP_VTABLE);
}

// Translation incomplete: 0044c480 - the C++ exception frame (FS:[0]) and its unwind actions are not translated
// Translated from 0044c480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the map (`thiscall(this, keep_none)`), inside a scope object:
/// nothing when it has no storage object. Otherwise, under the lock, reads
/// the storage's first word (its size), destroys the storage, clears +0x14
/// and +0x18, and for every bucket clears the bucket word and frees each of
/// its nodes (following each node's marked link at +8, clearing that link
/// and the node's +4 before [`MAP_FREE_NODE`]). When `keep_none` is 0 a new
/// storage object of the old size is built in its place.
pub fn fn_0044c480(e: &mut Engine, this: Ptr, keep_none: u8) {
    let this_addr = this.addr();
    e.with_stack(4, |e, scope| {
        e.call(
            SCOPE_OPEN,
            &args![scope, 6u32, 1u32, THREAD_SAFE_STRUCTURES_FILE, 0x42cu32],
        );
        if e.mem.u32(this_addr + 0x14) == 0 {
            e.call(SCOPE_CLOSE, &args![scope]);
            return;
        }
        e.call(LOCK, &args![this_addr + 0x20, 0u32]);
        let storage = e.mem.u32(this_addr + 0x14);
        let size = e.call(STORAGE_FIRST_WORD, &args![storage]).u32();
        let storage = e.mem.u32(this_addr + 0x14);
        if storage != 0 {
            e.call(STORAGE_DESTROY, &args![storage, 1u32]);
        }
        e.mem.set_u32(this_addr + 0x14, 0);
        e.mem.set_u32(this_addr + 0x18, 0);
        let mut index = 0u32;
        while index < e.mem.u32(this_addr + 8) {
            let bucket = e
                .mem
                .u32(this_addr + 0xc)
                .wrapping_add(index.wrapping_mul(4));
            let mut node = e.call(MARKED_POINTER_GET, &args![bucket]).u32();
            e.call(MARKED_POINTER_CLEAR, &args![bucket]);
            while node != 0 {
                let link = node.wrapping_add(8);
                let next = e.call(MARKED_POINTER_GET, &args![link]).u32();
                e.call(MARKED_POINTER_CLEAR, &args![link]);
                e.mem.set_u32(node.wrapping_add(4), 0);
                e.call(MAP_FREE_NODE, &args![this, node]);
                node = next;
            }
            index = index.wrapping_add(1);
        }
        if keep_none == 0 {
            let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
            let storage = if block == 0 {
                0
            } else {
                e.call(STORAGE_CONSTRUCT, &args![block, size]).u32()
            };
            e.mem.set_u32(this_addr + 0x14, storage);
        }
        e.call(UNLOCK, &args![this_addr + 0x20]);
        e.call(SCOPE_CLOSE, &args![scope]);
    });
}

// Translated from 0044c640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Advances an iterator over the map (`thiscall(this, iterator, out_key,
/// extra, keep_going)`), under the lock: nothing (0) when the iterator is at
/// its end (bit 1). Otherwise it steps through the bucket the iterator is at
/// (`[iterator + 4]`): the first step when its bit 0 is clear (then sets bit
/// 0), the next step when it is set, each on the lookup target with the
/// bucket, `iterator + 8` and `extra`. A step that answers non-zero stores
/// `[iterator + 8]` in `out_key`; otherwise the iterator moves to the next
/// bucket: past the last one it is marked at its end (bit 1); with
/// `keep_going` it clears bit 0 and steps again. Returns the last step's
/// answer.
pub fn fn_0044c640(
    e: &mut Engine,
    this: Ptr,
    iterator: Ptr,
    out_key: Ptr,
    extra: u32,
    keep_going: u8,
) -> u8 {
    let this_addr = this.addr();
    let iterator_addr = iterator.addr();
    e.call(LOCK, &args![this_addr + 0x20, 0u32]);
    let mut found = 0u8;
    if !e.call(ITERATOR_AT_END, &args![iterator]).bool() {
        loop {
            let bit_0 = e.call(ITERATOR_BIT_0, &args![iterator]).bool();
            let bucket = e.mem.u32(iterator_addr + 4);
            let target = e.call(MAP_LOOKUP_TARGET, &args![this]).u32();
            let step = if bit_0 {
                BUCKET_STEP_NEXT
            } else {
                BUCKET_STEP_FIRST
            };
            found = e
                .call(step, &args![target, bucket, iterator_addr + 8, extra])
                .u8();
            if !bit_0 {
                e.call(ITERATOR_SET_BIT_0, &args![iterator, 1u32]);
            }
            if found != 0 {
                let key = e.mem.u32(iterator_addr + 8);
                e.mem.set_u32(out_key.addr(), key);
                break;
            }
            let next = e.mem.u32(iterator_addr + 4).wrapping_add(1);
            e.mem.set_u32(iterator_addr + 4, next);
            let count = e.call(MAP_BUCKET_COUNT, &args![this]).u32();
            if e.mem.u32(iterator_addr + 4) >= count {
                e.call(ITERATOR_SET_BIT_1, &args![iterator, 1u32]);
                break;
            }
            if keep_going == 0 {
                break;
            }
            e.call(ITERATOR_SET_BIT_0, &args![iterator, 0u32]);
        }
    }
    e.call(UNLOCK, &args![this_addr + 0x20]);
    found
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0044c000, fn_0044c000(Ptr, u32, u32, u8) -> u32),
        entry!(0x0044c040, fn_0044c040(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x0044c1c0, fn_0044c1c0(Ptr, u32, u32) -> bool),
        entry!(0x0044c1e0, fn_0044c1e0(Ptr)),
        entry!(0x0044c270, fn_0044c270(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x0044c3f0, fn_0044c3f0(Ptr)),
        entry!(0x0044c480, fn_0044c480(Ptr, u8)),
        entry!(0x0044c640, fn_0044c640(Ptr, Ptr, Ptr, u32, u8) -> u8),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn stub(e: &mut Engine, address: u32) {
        e.register(address, |_, _| Ret::default());
    }

    fn returning(e: &mut Engine, address: u32, value: u32) {
        e.register_double(address, move |_, _| Ret {
            eax: value,
            ..Ret::default()
        });
    }

    #[test]
    fn insert_hashes_through_the_pointed_object() {
        let mut e = Engine::new();
        let this = e.mem.alloc(4);
        let object = e.mem.alloc(4);
        e.mem.set_u32(this, object);
        e.put_vtable(0x0090_0000, &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0x0090_1000]);
        e.mem.set_u32(object, 0x0090_0000);
        e.register(0x0090_1000, |_, a| Ret {
            eax: a[1] * 3,
            ..Ret::default()
        });
        e.register(MAP_INSERT, |_, a| Ret {
            eax: 0x100 + a[4],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let r = e
            .call(0x0044_c000, &args![this, 7u32, 9u32, 0x1ffu32])
            .u32();
        assert_eq!(r, 0x1ff);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0090_1000), vec![vec![object, 7]]);
        assert_eq!(calls_to(&log, MAP_INSERT), vec![vec![this, 21, 7, 9, 0xff]]);
    }

    fn constructor_doubles(e: &mut Engine, fail_storage: bool) {
        for address in [
            MAP_BASE_CONSTRUCT,
            LOCK_CONSTRUCT,
            SCOPE_OPEN,
            SCOPE_CLOSE,
            VECTOR_CONSTRUCT,
        ] {
            stub(e, address);
        }
        e.register_double(OPERATOR_NEW, move |_, a| Ret {
            eax: if a[0] == 0x10 && fail_storage {
                0
            } else {
                0x0700_0000u32.wrapping_add(a[0])
            },
            ..Ret::default()
        });
        returning(e, STORAGE_CONSTRUCT, 0x0800_0000);
    }

    #[test]
    fn constructors_build_arrays_and_storage() {
        for (address, vtable) in [
            (0x0044_c040u32, MODEL_MAP_VTABLE),
            (0x0044_c270, KF_MODEL_MAP_VTABLE),
        ] {
            let mut e = Engine::new();
            constructor_doubles(&mut e, false);
            let this = e.mem.alloc(0x24);
            e.mem.set_u32(this + 0x18, 5);
            e.call_log = Some(vec![]);
            let r = e.call(address, &args![this, 10u32, 4u32, 12u32]).u32();
            assert_eq!(r, this);
            assert_eq!(e.mem.u32(this), vtable);
            assert_eq!(e.mem.u32(this + 4), 0x0700_0000 + 120);
            assert_eq!(e.mem.u32(this + 8), 4);
            assert_eq!(e.mem.u32(this + 0xc), 0x0700_0000 + 16);
            assert_eq!(e.mem.u32(this + 0x10), 12);
            assert_eq!(e.mem.u32(this + 0x14), 0x0800_0000);
            assert_eq!(e.mem.u32(this + 0x18), 0);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls_to(&log, MAP_BASE_CONSTRUCT), vec![vec![this]]);
            assert_eq!(calls_to(&log, LOCK_CONSTRUCT), vec![vec![this + 0x20]]);
            let scope = calls_to(&log, SCOPE_OPEN)[0].clone();
            assert_eq!(&scope[1..], &[6, 1, THREAD_SAFE_STRUCTURES_FILE, 0x390]);
            assert_eq!(calls_to(&log, SCOPE_CLOSE), vec![vec![scope[0]]]);
            assert_eq!(
                calls_to(&log, VECTOR_CONSTRUCT),
                vec![vec![0x0700_0010, 4, 4, BUCKET_CONSTRUCT]]
            );
            assert_eq!(
                calls_to(&log, STORAGE_CONSTRUCT),
                vec![vec![0x0700_0010, 10]]
            );
        }
    }

    #[test]
    fn constructor_saturates_overflowing_sizes_and_survives_failed_storage() {
        let mut e = Engine::new();
        constructor_doubles(&mut e, true);
        let this = e.mem.alloc(0x24);
        e.call_log = Some(vec![]);
        e.call(
            0x0044_c040,
            &args![this, 0x2000_0000u32, 0x4000_0000u32, 0u32],
        );
        let log = e.call_log.take().unwrap();
        let sizes: Vec<u32> = calls_to(&log, OPERATOR_NEW).iter().map(|a| a[0]).collect();
        // 0x40000000 * 4 overflows; 3 * 0x20000000 = 0x60000000, * 4 overflows.
        assert_eq!(sizes, vec![u32::MAX, u32::MAX, 0x10]);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert!(calls_to(&log, STORAGE_CONSTRUCT).is_empty());
        assert_eq!(allocation_size(0x1000, 4), 0x4000);
    }

    #[test]
    fn key_comparison_is_unsigned_greater_or_equal() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0044_c1c0, &args![0u32, 5u32, 5u32]).u32(), 1);
        assert_eq!(e.call(0x0044_c1c0, &args![0u32, 4u32, 5u32]).u32(), 0);
        assert_eq!(e.call(0x0044_c1c0, &args![0u32, u32::MAX, 5u32]).u32(), 1);
    }

    #[test]
    fn destructors_empty_free_and_destroy_the_base() {
        for (address, vtable) in [
            (0x0044_c1e0u32, MODEL_MAP_VTABLE),
            (0x0044_c3f0, KF_MODEL_MAP_VTABLE),
        ] {
            let mut e = Engine::new();
            for callee in [SCOPE_OPEN, SCOPE_CLOSE, OPERATOR_DELETE, MAP_BASE_DESTROY] {
                stub(&mut e, callee);
            }
            let this = e.mem.alloc(0x24);
            e.mem.set_u32(this + 4, 0x0700_0100);
            e.mem.set_u32(this + 0xc, 0x0700_0200);
            e.call_log = Some(vec![]);
            e.call(address, &args![this]);
            assert_eq!(e.mem.u32(this), vtable);
            let log = e.call_log.take().unwrap();
            let freed = calls_to(&log, OPERATOR_DELETE);
            assert_eq!(freed, vec![vec![0x0700_0200], vec![0x0700_0100]]);
            assert_eq!(calls_to(&log, MAP_BASE_DESTROY), vec![vec![this]]);
            // No storage: the emptying only opens and closes its scope.
            let scope = calls_to(&log, SCOPE_OPEN)[0].clone();
            assert_eq!(scope[4], 0x42c);
            assert_eq!(calls_to(&log, SCOPE_CLOSE).len(), 1);
        }
    }

    fn clear_doubles(e: &mut Engine) {
        for address in [
            SCOPE_OPEN,
            SCOPE_CLOSE,
            LOCK,
            UNLOCK,
            STORAGE_DESTROY,
            MAP_FREE_NODE,
        ] {
            stub(e, address);
        }
        returning(e, STORAGE_FIRST_WORD, 33);
        e.register(MARKED_POINTER_GET, |e, a| Ret {
            eax: e.mem.u32(a[0]) & !1,
            ..Ret::default()
        });
        e.register(MARKED_POINTER_CLEAR, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        e.register(OPERATOR_NEW, |_, _| Ret {
            eax: 0x0700_0000,
            ..Ret::default()
        });
        returning(e, STORAGE_CONSTRUCT, 0x0800_0000);
    }

    #[test]
    fn emptying_frees_every_node_and_rebuilds_the_storage() {
        for keep_none in [0u8, 1] {
            let mut e = Engine::new();
            clear_doubles(&mut e);
            let this = e.mem.alloc(0x24);
            let buckets = e.mem.alloc(8);
            let node_a = e.mem.alloc(0xc);
            let node_b = e.mem.alloc(0xc);
            let node_c = e.mem.alloc(0xc);
            e.mem.set_u32(buckets, node_a | 1);
            e.mem.set_u32(node_a + 8, node_b);
            e.mem.set_u32(node_a + 4, 0x55);
            e.mem.set_u32(buckets + 4, node_c);
            e.mem.set_u32(this + 8, 2);
            e.mem.set_u32(this + 0xc, buckets);
            e.mem.set_u32(this + 0x14, 0x0600_0000);
            e.mem.set_u32(this + 0x18, 9);
            e.call_log = Some(vec![]);
            e.call(0x0044_c480, &args![this, keep_none]);
            let log = e.call_log.take().unwrap();
            assert_eq!(
                calls_to(&log, MAP_FREE_NODE),
                vec![vec![this, node_a], vec![this, node_b], vec![this, node_c]]
            );
            assert_eq!(e.mem.u32(node_a + 4), 0);
            assert_eq!(e.mem.u32(node_a + 8), 0);
            assert_eq!(e.mem.u32(buckets), 0);
            assert_eq!(calls_to(&log, STORAGE_DESTROY), vec![vec![0x0600_0000, 1]]);
            assert_eq!(calls_to(&log, LOCK), vec![vec![this + 0x20, 0]]);
            assert_eq!(calls_to(&log, UNLOCK), vec![vec![this + 0x20]]);
            assert_eq!(e.mem.u32(this + 0x18), 0);
            if keep_none == 0 {
                assert_eq!(
                    calls_to(&log, STORAGE_CONSTRUCT),
                    vec![vec![0x0700_0000, 33]]
                );
                assert_eq!(e.mem.u32(this + 0x14), 0x0800_0000);
            } else {
                assert!(calls_to(&log, STORAGE_CONSTRUCT).is_empty());
                assert_eq!(e.mem.u32(this + 0x14), 0);
            }
        }
    }

    #[test]
    fn emptying_without_storage_does_not_lock() {
        let mut e = Engine::new();
        clear_doubles(&mut e);
        let this = e.mem.alloc(0x24);
        e.call_log = Some(vec![]);
        e.call(0x0044_c480, &args![this, 0u8]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, LOCK).is_empty());
        assert_eq!(calls_to(&log, SCOPE_CLOSE).len(), 1);
    }

    /// Iterator doubles: the iterator's byte at +0x0c holds the bits; the
    /// steps answer from `answers` in turn.
    fn iterator_doubles(e: &mut Engine, answers: Vec<u8>) {
        for address in [LOCK, UNLOCK] {
            stub(e, address);
        }
        e.register(ITERATOR_AT_END, |e, a| {
            (e.mem.u8(a[0] + 0xc) & 2 != 0).into_ret()
        });
        e.register(ITERATOR_BIT_0, |e, a| {
            (e.mem.u8(a[0] + 0xc) & 1 != 0).into_ret()
        });
        e.register(ITERATOR_SET_BIT_0, |e, a| {
            let bits = e.mem.u8(a[0] + 0xc);
            let bits = if a[1] as u8 != 0 { bits | 1 } else { bits & !1 };
            e.mem.set_u8(a[0] + 0xc, bits);
            Ret::default()
        });
        e.register(ITERATOR_SET_BIT_1, |e, a| {
            let bits = e.mem.u8(a[0] + 0xc) | 2;
            e.mem.set_u8(a[0] + 0xc, bits);
            Ret::default()
        });
        e.register(MAP_LOOKUP_TARGET, |_, a| Ret {
            eax: a[0] + 0x1000,
            ..Ret::default()
        });
        e.register(MAP_BUCKET_COUNT, |_, _| Ret {
            eax: 3,
            ..Ret::default()
        });
        let mut answers = answers.into_iter();
        let step = move |_: &mut Engine, _: &[u32]| Ret {
            eax: u32::from(answers.next().unwrap_or(0)),
            ..Ret::default()
        };
        let shared = std::rc::Rc::new(std::cell::RefCell::new(step));
        let first = shared.clone();
        e.register_double(BUCKET_STEP_FIRST, move |e, a| (first.borrow_mut())(e, a));
        e.register_double(BUCKET_STEP_NEXT, move |e, a| (shared.borrow_mut())(e, a));
    }

    #[test]
    fn iterator_steps_until_a_match_and_reports_the_key() {
        let mut e = Engine::new();
        iterator_doubles(&mut e, vec![0, 0, 7]);
        let this = e.mem.alloc(0x24);
        let iterator = e.mem.alloc(0x10);
        let out = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        let r = e.call(0x0044_c640, &args![this, iterator, out, 0x44u32, 1u8]);
        assert_eq!(r.u8(), 7);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, BUCKET_STEP_FIRST),
            vec![
                vec![this + 0x1000, 0, iterator + 8, 0x44],
                vec![this + 0x1000, 1, iterator + 8, 0x44],
                vec![this + 0x1000, 2, iterator + 8, 0x44],
            ]
        );
        e.mem.set_u32(iterator + 8, 0x99);
        // Bit 0 is now set: the next call takes the other step.
        iterator_doubles(&mut e, vec![5]);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0044_c640, &args![this, iterator, out, 1u32, 0u8])
                .u8(),
            5
        );
        assert_eq!(e.mem.u32(out), 0x99);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, BUCKET_STEP_NEXT).len(), 1);
        assert!(calls_to(&log, BUCKET_STEP_FIRST).is_empty());
        assert_eq!(calls_to(&log, UNLOCK), vec![vec![this + 0x20]]);
    }

    #[test]
    fn iterator_stops_at_the_end_or_without_keep_going() {
        let mut e = Engine::new();
        iterator_doubles(&mut e, vec![]);
        let this = e.mem.alloc(0x24);
        let iterator = e.mem.alloc(0x10);
        let out = e.mem.alloc(4);
        // Without keep_going: one step, the bucket advances, bit 0 stays set.
        let r = e.call(0x0044_c640, &args![this, iterator, out, 0u32, 0u8]);
        assert_eq!(r.u8(), 0);
        assert_eq!(e.mem.u32(iterator + 4), 1);
        assert_eq!(e.mem.u8(iterator + 0xc), 1);
        // With keep_going: runs past the last bucket and marks the end.
        e.call(0x0044_c640, &args![this, iterator, out, 0u32, 1u8]);
        assert_eq!(e.mem.u32(iterator + 4), 3);
        assert_eq!(e.mem.u8(iterator + 0xc) & 2, 2);
        // At the end: no step at all.
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0044_c640, &args![this, iterator, out, 0u32, 1u8])
                .u8(),
            0
        );
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, MAP_LOOKUP_TARGET).is_empty());
        assert_eq!(calls_to(&log, LOCK).len(), 1);
        assert_eq!(calls_to(&log, UNLOCK).len(), 1);
        assert_eq!(e.mem.u32(out), 0);
    }
}
