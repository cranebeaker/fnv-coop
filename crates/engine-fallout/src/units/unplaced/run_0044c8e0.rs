//! `(unplaced)/run_0044c8e0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Members of `LockFreeMap<const char *, LoadedFile *>` (the RTTI of its
//! vtable `010176ac` reads `.?AV?$LockFreeMap@PBDPAVLoadedFile@@@@`): an
//! insert-style wrapper, the constructor, the bucket index (vtable slot
//! 0x24), a by-pointer copy (slot 0x30) and the destructor.

#[allow(unused_imports)]
use crate::prelude::*;

/// The map's vtable (`LockFreeMap<const char *, LoadedFile *>`, RTTI).
const LOCK_FREE_MAP_VTABLE: u32 = 0x0101_76ac;
/// `0044d510` (`thiscall`, `RET 0xC`): called by [`fn_0044c8e0`] with the
/// bucket index and both of its arguments; returns a byte in `AL`.
const BUCKET_OPERATION: u32 = 0x0044_d510;
/// The base constructor (`0044d7f0`, `thiscall`, no stack words).
const BASE_CONSTRUCT: u32 = 0x0044_d7f0;
/// The constructor of the member at +0x20 (`0096a2d0`, `thiscall`).
const MEMBER_CONSTRUCT: u32 = 0x0096_a2d0;
/// A scope guard's constructor (`00404eb0`, `thiscall`, `RET 0x10`).
const GUARD_ENTER: u32 = 0x0040_4eb0;
/// The scope guard's destructor (`00404ee0`, `thiscall`).
const GUARD_LEAVE: u32 = 0x0040_4ee0;
/// The allocator (`00401000`, `cdecl` (size)).
const ALLOCATE: u32 = 0x0040_1000;
/// The deallocator (`00401030`, `cdecl` (block)).
const FREE: u32 = 0x0040_1030;
/// The array constructor iterator (`00401050`, `stdcall` (array, element
/// size, count, constructor)).
const CONSTRUCT_ARRAY: u32 = 0x0040_1050;
/// The bucket constructor passed to [`CONSTRUCT_ARRAY`] (`0044dee0`).
const BUCKET_CONSTRUCT: u32 = 0x0044_dee0;
/// The constructor of the 16-byte object at +0x14 (`006c73b0`, `thiscall`,
/// `RET 4`).
const SIXTEEN_BYTE_CONSTRUCT: u32 = 0x006c_73b0;
/// The source path string the guard receives (`010173f8`,
/// `...\bsmain\ThreadSafeStructures.inl`).
const SOURCE_PATH: u32 = 0x0101_73f8;
/// The clean-up called by the destructor (`0044c480`, `thiscall`, `RET 4`).
const CLEAR: u32 = 0x0044_c480;
/// The base destructor (`0044b4b0`, `thiscall`).
const BASE_DESTROY: u32 = 0x0044_b4b0;

/// The `operator new[]` size of `count` elements of `size` bytes: the
/// product, or `0xFFFFFFFF` when the 32-bit multiply overflows (`MUL`,
/// `SETO`, `NEG`, `OR`).
fn array_size(count: u32, size: u32) -> u32 {
    count.saturating_mul(size)
}

// Translated from 0044c8e0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// (`thiscall`, `RET 8`.) Calls slot 0x24 of the object whose address is the
/// word at `this` (with that word as `ECX`) with `key`, then `0044d510` on
/// `this` with that result, `key` and `value`, and returns its byte (`AL`,
/// which the callers keep).
pub fn fn_0044c8e0(e: &mut Engine, this: Ptr, key: u32, value: u32) -> u8 {
    let object = e.mem.u32(this.addr());
    let index = e.vcall(object, 0x24, &args![key]).u32();
    e.call(BUCKET_OPERATION, &args![this, index, key, value])
        .u8()
}

// Translated from 0044c920 (PC disassembly, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0044c920 - the FS:[0] exception frame (unwinding of the guard and partial allocations) is not translated
/// The map's constructor (`thiscall`, `RET 0xC`): the base constructor, the
/// vtable `010176ac`, the member at +0x20, a scope guard (6, 1, the source
/// path, line 0x390); then +0x18 = 0, the bucket count at +8, a bucket
/// array of `count * 4` bytes built with `0044dee0` per bucket at +0xC (0
/// when the allocation fails), an array of `capacity * 3 * 4` bytes at +4,
/// `third` at +0x10, and a 16-byte object built with `capacity`
/// (`006c73b0`) at +0x14 (0 when that allocation fails). Leaves the guard
/// and returns `this`.
pub fn fn_0044c920(e: &mut Engine, this: Ptr, capacity: u32, buckets: u32, third: u32) -> Ptr {
    let t = this.addr();
    e.call(BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(t, LOCK_FREE_MAP_VTABLE);
    e.call(MEMBER_CONSTRUCT, &args![t + 0x20]);
    e.with_stack(4, |e, guard| {
        e.call(
            GUARD_ENTER,
            &args![guard, 6u32, 1u32, SOURCE_PATH, 0x390u32],
        );
        e.mem.set_u32(t + 0x18, 0);
        e.mem.set_u32(t + 8, buckets);
        let count = e.mem.u32(t + 8);
        let block = e.call(ALLOCATE, &args![array_size(count, 4)]).u32();
        let array = if block != 0 {
            e.call(
                CONSTRUCT_ARRAY,
                &args![block, 4u32, count, BUCKET_CONSTRUCT],
            );
            block
        } else {
            0
        };
        e.mem.set_u32(t + 0xc, array);
        let size = array_size(capacity.wrapping_mul(3), 4);
        let entries = e.call(ALLOCATE, &args![size]).u32();
        e.mem.set_u32(t + 4, entries);
        e.mem.set_u32(t + 0x10, third);
        let block = e.call(ALLOCATE, &args![0x10u32]).u32();
        let object = if block != 0 {
            e.call(SIXTEEN_BYTE_CONSTRUCT, &args![block, capacity])
                .u32()
        } else {
            0
        };
        e.mem.set_u32(t + 0x14, object);
        e.call(GUARD_LEAVE, &args![guard]);
    });
    this
}

// Translated from 0044caa0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The bucket index (`thiscall`, `RET 4`): `key` modulo the bucket count at
/// +8 (`DIV`; a zero count faults in the game as here).
pub fn fn_0044caa0(e: &mut Engine, this: Ptr, key: u32) -> u32 {
    key % e.mem.u32(this.addr() + 8)
}

// Translated from 0044cac0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// (`thiscall`, `RET 8`.) Stores `value` at `out`; `this` is not read.
pub fn fn_0044cac0(e: &mut Engine, _this: Ptr, value: u32, out: u32) {
    e.mem.set_u32(out, value);
}

// Translated from 0044cae0 (PC disassembly, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0044cae0 - the FS:[0] exception frame (unwinding to the base destructor) is not translated
/// The map's destructor (`thiscall`): the vtable `010176ac`, `0044c480`
/// with 1, frees the bucket array (+0xC) and the array at +4 (`00401030`),
/// then the base destructor `0044b4b0`.
pub fn fn_0044cae0(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    e.mem.set_u32(t, LOCK_FREE_MAP_VTABLE);
    e.call(CLEAR, &args![this, 1u32]);
    let buckets = e.mem.u32(t + 0xc);
    e.call(FREE, &args![buckets]);
    let entries = e.mem.u32(t + 4);
    e.call(FREE, &args![entries]);
    e.call(BASE_DESTROY, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0044c8e0, fn_0044c8e0(Ptr, u32, u32) -> u8),
        entry!(0x0044c920, fn_0044c920(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x0044caa0, fn_0044caa0(Ptr, u32) -> u32),
        entry!(0x0044cac0, fn_0044cac0(Ptr, u32, u32)),
        entry!(0x0044cae0, fn_0044cae0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_wrapper_passes_the_slot_result_on() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        let object = e.mem.alloc(8);
        e.mem.set_u32(this, object);
        e.put_vtable(0x0300_0000, &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0x0300_1000]);
        e.mem.set_u32(object, 0x0300_0000);
        e.register(0x0300_1000, |_, a| Ret {
            eax: a[1] + 100,
            ..Ret::default()
        });
        e.register(BUCKET_OPERATION, |_, _| Ret {
            eax: 0xabcd_ef01,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0044_c8e0, &[this, 7, 9]).u8(), 1);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(0x0300_1000, vec![object, 7])));
        assert!(log.contains(&(BUCKET_OPERATION, vec![this, 107, 7, 9])));
    }

    fn constructor_doubles(e: &mut Engine, fail_allocations: bool) {
        for a in [
            BASE_CONSTRUCT,
            MEMBER_CONSTRUCT,
            GUARD_ENTER,
            GUARD_LEAVE,
            CONSTRUCT_ARRAY,
        ] {
            e.register(a, |_, _| Ret::default());
        }
        e.register(SIXTEEN_BYTE_CONSTRUCT, |_, a| a[0].into_ret());
        if fail_allocations {
            e.register(ALLOCATE, |_, _| Ret::default());
        } else {
            e.register(ALLOCATE, |e, a| e.mem.alloc(a[0].min(0x100)).into_ret());
        }
    }

    #[test]
    fn the_constructor_builds_its_arrays() {
        let mut e = Engine::new();
        constructor_doubles(&mut e, false);
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0044_c920, &[this, 5, 3, 0x77]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), LOCK_FREE_MAP_VTABLE);
        assert_eq!(e.mem.u32(this + 8), 3);
        assert_eq!(e.mem.u32(this + 0x10), 0x77);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        let sizes: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == ALLOCATE)
            .map(|c| c.1[0])
            .collect();
        assert_eq!(sizes, vec![12, 60, 16]);
        let buckets = e.mem.u32(this + 0xc);
        assert_ne!(buckets, 0);
        assert!(log.contains(&(CONSTRUCT_ARRAY, vec![buckets, 4, 3, BUCKET_CONSTRUCT])));
        let object = e.mem.u32(this + 0x14);
        assert!(log.contains(&(SIXTEEN_BYTE_CONSTRUCT, vec![object, 5])));
        assert!(log.contains(&(MEMBER_CONSTRUCT, vec![this + 0x20])));
        let guard = log.iter().find(|c| c.0 == GUARD_ENTER).unwrap().1.clone();
        assert_eq!(guard[1..], [6, 1, SOURCE_PATH, 0x390]);
        assert!(log.contains(&(GUARD_LEAVE, vec![guard[0]])));
    }

    #[test]
    fn the_constructor_survives_failed_allocations_and_saturates_sizes() {
        let mut e = Engine::new();
        constructor_doubles(&mut e, true);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0xc, 0x1234);
        e.mem.set_u32(this + 0x14, 0x1234);
        e.call_log = Some(vec![]);
        e.call(0x0044_c920, &[this, 0x4000_0000, 0x4000_0000, 0]);
        let log = e.call_log.take().unwrap();
        let sizes: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == ALLOCATE)
            .map(|c| c.1[0])
            .collect();
        assert_eq!(sizes, vec![u32::MAX, u32::MAX, 16]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert!(!log
            .iter()
            .any(|c| c.0 == CONSTRUCT_ARRAY || c.0 == SIXTEEN_BYTE_CONSTRUCT));
    }

    #[test]
    fn the_bucket_index_is_the_key_modulo_the_count() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 8, 7);
        assert_eq!(
            e.call(0x0044_caa0, &[this, 0xffff_ffff]).u32(),
            0xffff_ffff % 7
        );
    }

    #[test]
    fn the_copy_stores_through_the_pointer() {
        let mut e = Engine::new();
        let out = e.mem.alloc(4);
        e.call(0x0044_cac0, &[0, 0x55, out]);
        assert_eq!(e.mem.u32(out), 0x55);
    }

    #[test]
    fn the_destructor_frees_both_arrays() {
        let mut e = Engine::new();
        for a in [CLEAR, FREE, BASE_DESTROY] {
            e.register(a, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 4, 0x4444);
        e.mem.set_u32(this + 0xc, 0xcccc);
        e.call_log = Some(vec![]);
        e.call(0x0044_cae0, &[this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), LOCK_FREE_MAP_VTABLE);
        let calls: Vec<_> = log.into_iter().skip(1).collect();
        assert_eq!(
            calls,
            vec![
                (CLEAR, vec![this, 1]),
                (FREE, vec![0xcccc]),
                (FREE, vec![0x4444]),
                (BASE_DESTROY, vec![this]),
            ]
        );
    }
}
