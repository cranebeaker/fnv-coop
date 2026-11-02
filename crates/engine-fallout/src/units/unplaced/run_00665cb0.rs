//! `(unplaced)/run_00665cb0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `LockFreeMap<TESObjectREFR*, BSTreeNode*>` (named by the RTTI locator of
//! the vtable `01066454` the constructor sets; fields from the Xbox PDB,
//! offsets as the PC code uses them): its constructor, `AllocateInterface`
//! (vtable +0x04), `CopyKeyDirect` (+0x2C) and a helper of its interface.

#[allow(unused_imports)]
use crate::prelude::*;

/// `LockFreeMap<TESObjectREFR*, BSTreeNode*>`'s vtable.
const LOCK_FREE_MAP_VTABLE: u32 = 0x0106_6454;
/// The base constructor (`0044d7f0`, `thiscall`).
const BASE_CONSTRUCT: u32 = 0x0044_d7f0;
/// The two-word list head constructor (`0096a2d0`, `thiscall`).
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// The scope guard's constructor (`00404eb0`, `thiscall(value, mode, source
/// file, line)`) and destructor (`00404ee0`).
const SCOPE_GUARD_CONSTRUCT: u32 = 0x0040_4eb0;
const SCOPE_GUARD_DESTRUCT: u32 = 0x0040_4ee0;
/// `"d:\_fallout3\platforms\common\code\bsmain\ThreadSafeStructures.inl"`
/// and `"..\BSMain\ThreadSafeStructures.h"`.
const STRUCTURES_INL: u32 = 0x0101_73f8;
const STRUCTURES_H: u32 = 0x0106_649c;
/// `operator new` (`00401000`, `cdecl(size)`).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// The vector constructor iterator (`00401050`, `stdcall(block, size,
/// count, constructor)`) and the bucket constructor it is given
/// (`0044dee0`).
const VECTOR_CONSTRUCT: u32 = 0x0040_1050;
const BUCKET_CONSTRUCT: u32 = 0x0044_dee0;
/// The interface manager's constructor (`006c73b0`, `thiscall(count)`).
const INTERFACE_MANAGER_CONSTRUCT: u32 = 0x006c_73b0;
/// The interface's constructor (`0044d000`, `thiscall(map, three node
/// pointers)`).
const INTERFACE_CONSTRUCT: u32 = 0x0044_d000;
/// The helper `00665f20` hands the slot's result and the key to (`00666080`,
/// `thiscall(a, key)`).
const KEY_HELPER: u32 = 0x0066_6080;

layout! {
    /// `LockFreeMap<TESObjectREFR*, BSTreeNode*>` (Xbox PDB, 0x1C bytes
    /// there); the PC constructor also builds a list head at +0x20.
    pub struct LockFreeMap: 0x28 {
        /// `pReferencedNodes` (Xbox PDB): three words per interface.
        0x04 pReferencedNodes: u32,
        /// `iNumBuckets` (Xbox PDB).
        0x08 iNumBuckets: u32,
        /// `pBuckets` (Xbox PDB).
        0x0C pBuckets: u32,
        /// `iDeleteBatchSize` (Xbox PDB).
        0x10 iDeleteBatchSize: u32,
        /// `pInterfaceManager` (Xbox PDB).
        0x14 pInterfaceManager: u32,
        /// `iCount` (Xbox PDB).
        0x18 iCount: i32,
    }
}

/// `MUL` by 4 with the compiler's overflow guard (`SETO`/`NEG`/`OR`): all
/// ones when the product does not fit.
fn words_size(count: u32) -> u32 {
    count.checked_mul(4).unwrap_or(u32::MAX)
}

// Translated from 00665cb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00665cb0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Constructor (`thiscall(interfaces, buckets, delete batch size)`, `RET
/// 0xC`): base part (`0044d7f0`), vtable `01066454`, the list at +0x20,
/// then under a scope guard (`ThreadSafeStructures.inl`, line 0x390):
/// `iCount = 0`, `iNumBuckets = buckets`, the bucket array (`buckets`
/// words, each built by `0044dee0`; null when the allocation fails), the
/// referenced nodes (`interfaces * 3` words), `iDeleteBatchSize`, and the
/// interface manager (0x10 bytes, `006c73b0(interfaces)`; null when the
/// allocation fails). Returns `this`.
pub fn fn_00665cb0(
    e: &mut Engine,
    this: Ptr<LockFreeMap>,
    interfaces: u32,
    buckets: u32,
    delete_batch_size: u32,
) -> Ptr<LockFreeMap> {
    e.call(BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), LOCK_FREE_MAP_VTABLE);
    e.call(LIST_CONSTRUCT, &args![this.addr() + 0x20]);
    e.with_stack(4, |e, guard| {
        e.call(
            SCOPE_GUARD_CONSTRUCT,
            &args![guard, 6u32, 1u32, STRUCTURES_INL, 0x390u32],
        );
        e.set(this, LockFreeMap::iCount, 0);
        e.set(this, LockFreeMap::iNumBuckets, buckets);
        let count = e.get(this, LockFreeMap::iNumBuckets);
        let block = e.call(OPERATOR_NEW, &args![words_size(count)]).u32();
        let array = if block != 0 {
            e.call(
                VECTOR_CONSTRUCT,
                &args![block, 4u32, count, BUCKET_CONSTRUCT],
            );
            block
        } else {
            0
        };
        e.set(this, LockFreeMap::pBuckets, array);
        let nodes = e
            .call(OPERATOR_NEW, &args![words_size(interfaces.wrapping_mul(3))])
            .u32();
        e.set(this, LockFreeMap::pReferencedNodes, nodes);
        e.set(this, LockFreeMap::iDeleteBatchSize, delete_batch_size);
        let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
        let manager = if block != 0 {
            e.call(INTERFACE_MANAGER_CONSTRUCT, &args![block, interfaces])
                .u32()
        } else {
            0
        };
        e.set(this, LockFreeMap::pInterfaceManager, manager);
        e.call(SCOPE_GUARD_DESTRUCT, &args![guard]);
    });
    this
}

// Translated from 00665e30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00665e30 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `AllocateInterface` (vtable +0x04; Xbox PDB slot name): under a scope
/// guard (`ThreadSafeStructures.h`, line 0x18E) makes the interface for
/// thread `index` (0x24 bytes, `0044d000(this, &nodes[3i], &nodes[3i+1],
/// &nodes[3i+2])`). Returns it, or null when the allocation fails.
pub fn fn_00665e30(e: &mut Engine, this: Ptr<LockFreeMap>, index: u32) -> u32 {
    e.with_stack(4, |e, guard| {
        e.call(
            SCOPE_GUARD_CONSTRUCT,
            &args![guard, 6u32, 1u32, STRUCTURES_H, 0x18eu32],
        );
        let block = e.call(OPERATOR_NEW, &args![0x24u32]).u32();
        let interface = if block != 0 {
            let at = |e: &Engine, extra: u32| {
                e.get(this, LockFreeMap::pReferencedNodes)
                    .wrapping_add(index.wrapping_mul(3).wrapping_mul(4))
                    .wrapping_add(extra)
            };
            let third = at(e, 8);
            let second = at(e, 4);
            let first = at(e, 0);
            e.call(
                INTERFACE_CONSTRUCT,
                &args![block, this, first, second, third],
            )
            .u32()
        } else {
            0
        };
        e.call(SCOPE_GUARD_DESTRUCT, &args![guard]);
        interface
    })
}

// Translated from 00665f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CopyKeyDirect` (vtable +0x2C; Xbox PDB slot name): returns the key.
pub fn fn_00665f10(_e: &mut Engine, _this: Ptr<LockFreeMap>, key: u32) -> u32 {
    key
}

// Translated from 00665f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall(key)`, `RET 4`): calls slot 0x24 of
/// the object `this` points to with `key`, then `00666080(this, result,
/// key)`, whose answer it answers (the exe ends on that call and leaves its
/// `EAX`; the callers keep `AL`).
pub fn fn_00665f20(e: &mut Engine, this: Ptr, key: u32) -> bool {
    let map = e.mem.u32(this.addr());
    let result = e.vcall(map, 0x24, &args![key]).u32();
    e.call(KEY_HELPER, &args![this, result, key]).bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0066_5cb0,
            fn_00665cb0(Ptr<LockFreeMap>, u32, u32, u32) -> Ptr<LockFreeMap>
        ),
        entry!(0x0066_5e30, fn_00665e30(Ptr<LockFreeMap>, u32) -> u32),
        entry!(0x0066_5f10, fn_00665f10(Ptr<LockFreeMap>, u32) -> u32),
        entry!(0x0066_5f20, fn_00665f20(Ptr, u32) -> bool),
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
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn engine(fail: bool) -> Engine {
        let mut e = Engine::new();
        for addr in [
            BASE_CONSTRUCT,
            LIST_CONSTRUCT,
            SCOPE_GUARD_CONSTRUCT,
            SCOPE_GUARD_DESTRUCT,
            VECTOR_CONSTRUCT,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        if fail {
            e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        } else {
            e.register(OPERATOR_NEW, |e, a| {
                e.mem.alloc(a[0].min(0x1000)).into_ret()
            });
        }
        e.register(INTERFACE_MANAGER_CONSTRUCT, |_, a| (a[0] + 1).into_ret());
        e.register(INTERFACE_CONSTRUCT, |_, a| (a[0] + 2).into_ret());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn constructor_fills_the_fields() {
        let mut e = engine(false);
        let this: Ptr<LockFreeMap> = e.new_object();
        e.set(this, LockFreeMap::iCount, 5);
        let back = e.call(0x0066_5cb0, &args![this, 4u32, 0x20u32, 7u32]).u32();
        assert_eq!(back, this.addr());
        assert_eq!(e.mem.u32(this.addr()), LOCK_FREE_MAP_VTABLE);
        assert_eq!(e.get(this, LockFreeMap::iCount), 0);
        assert_eq!(e.get(this, LockFreeMap::iNumBuckets), 0x20);
        assert_eq!(e.get(this, LockFreeMap::iDeleteBatchSize), 7);
        let sizes: Vec<u32> = calls(&e, OPERATOR_NEW).iter().map(|w| w[0]).collect();
        assert_eq!(sizes, vec![0x80, 0x30, 0x10]);
        let buckets = e.get(this, LockFreeMap::pBuckets);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![vec![buckets, 4, 0x20, BUCKET_CONSTRUCT]]
        );
        let manager = e.get(this, LockFreeMap::pInterfaceManager);
        assert_eq!(
            calls(&e, INTERFACE_MANAGER_CONSTRUCT),
            vec![vec![manager - 1, 4]]
        );
        assert_ne!(e.get(this, LockFreeMap::pReferencedNodes), 0);
        assert_eq!(calls(&e, LIST_CONSTRUCT), vec![vec![this.addr() + 0x20]]);
        let guard = calls(&e, SCOPE_GUARD_CONSTRUCT);
        assert_eq!(&guard[0][1..], &[6, 1, STRUCTURES_INL, 0x390]);
    }

    #[test]
    fn constructor_keeps_nulls_when_allocations_fail_and_guards_overflow() {
        let mut e = engine(true);
        let this: Ptr<LockFreeMap> = e.new_object();
        e.call(
            0x0066_5cb0,
            &args![this, 0x6000_0000u32, 0x4000_0000u32, 1u32],
        );
        let sizes: Vec<u32> = calls(&e, OPERATOR_NEW).iter().map(|w| w[0]).collect();
        // 0x40000000 * 4 overflows; 0x60000000 * 3 wraps to 0x20000000,
        // whose * 4 does not.
        assert_eq!(sizes, vec![u32::MAX, 0x8000_0000, 0x10]);
        assert!(calls(&e, VECTOR_CONSTRUCT).is_empty());
        assert!(calls(&e, INTERFACE_MANAGER_CONSTRUCT).is_empty());
        assert_eq!(e.get(this, LockFreeMap::pBuckets), 0);
        assert_eq!(e.get(this, LockFreeMap::pInterfaceManager), 0);
    }

    #[test]
    fn allocate_interface_points_at_the_threads_nodes() {
        let mut e = engine(false);
        let this: Ptr<LockFreeMap> = e.new_object();
        e.set(this, LockFreeMap::pReferencedNodes, 0x9000);
        let interface = e.call(0x0066_5e30, &args![this, 2u32]).u32();
        let made = calls(&e, INTERFACE_CONSTRUCT);
        assert_eq!(interface, made[0][0] + 2);
        assert_eq!(&made[0][1..], &[this.addr(), 0x9018, 0x901c, 0x9020]);
        let guard = calls(&e, SCOPE_GUARD_CONSTRUCT);
        assert_eq!(&guard[0][1..], &[6, 1, STRUCTURES_H, 0x18e]);
        assert_eq!(calls(&e, SCOPE_GUARD_DESTRUCT), vec![vec![guard[0][0]]]);
        let mut e = engine(true);
        assert_eq!(e.call(0x0066_5e30, &args![this, 2u32]).u32(), 0);
        assert!(calls(&e, INTERFACE_CONSTRUCT).is_empty());
    }

    #[test]
    fn copy_key_direct_returns_the_key() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0066_5f10, &args![0x100u32, 0xabcu32]).u32(), 0xabc);
    }

    #[test]
    fn helper_passes_the_slot_result_and_key() {
        let mut e = Engine::new();
        let map = e.mem.alloc(4);
        let vtable = e.mem.alloc(0x28);
        e.mem.set_u32(map, vtable);
        e.mem.set_u32(vtable + 0x24, 0x0066_0000);
        e.register(0x0066_0000, |_, a| (a[1] * 2).into_ret());
        e.register(KEY_HELPER, |_, _| Ret::default());
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, map);
        e.call_log = Some(vec![]);
        e.call(0x0066_5f20, &args![this, 0x21u32]);
        assert_eq!(calls(&e, 0x0066_0000), vec![vec![map, 0x21]]);
        assert_eq!(calls(&e, KEY_HELPER), vec![vec![this, 0x42, 0x21]]);
    }

    #[test]
    fn helper_answer_is_passed_back() {
        let mut e = Engine::new();
        let map = e.mem.alloc(4);
        let vtable = e.mem.alloc(0x28);
        e.mem.set_u32(map, vtable);
        e.mem.set_u32(vtable + 0x24, 0x0066_0000);
        e.register(0x0066_0000, |_, a| (a[1] * 2).into_ret());
        e.register(KEY_HELPER, |_, _| true.into_ret());
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, map);
        assert!(e.call(0x0066_5f20, &args![this, 0x21u32]).bool());
    }
}
