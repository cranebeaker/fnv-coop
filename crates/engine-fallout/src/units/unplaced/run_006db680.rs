//! `(unplaced)/run_006db680` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable `006db720` sets (RTTI `.?AV?$BSSimpleArray@M$0EAA@@@`, `BSSimpleArray<float,1024>`).
const SIMPLE_ARRAY_OF_FLOATS_VTABLE: u32 = 0x0106_cc64;
/// `008454f0` (`thiscall`, `RET 4`, no unit in the engine map): run with 1.
const ARRAY_008454F0: u32 = 0x0084_54f0;

// Translated from 006db720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; by its vtable the `BSSimpleArray<float,1024>` destructor body
/// (`ECX` only): sets the vtable `0106cc64` and runs `008454f0` on `this` with 1.
pub fn fn_006db720(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), SIMPLE_ARRAY_OF_FLOATS_VTABLE);
    e.call(ARRAY_008454F0, &args![this, 1u32]);
}

/// `BSScrapArray<float,1024>`'s vtable (RTTI `.?AV?$BSScrapArray@M$0EAA@@@`).
const SCRAP_ARRAY_OF_FLOATS_VTABLE: u32 = 0x0106_cc50;
/// `006dc2f0` (`thiscall`, no stack words): the `BSSimpleArray<float,1024>` base
/// constructor (stores `0106cc64` and initialises the array with `006b3eb0(0, 0)`).
const SIMPLE_ARRAY_OF_FLOATS_CONSTRUCT: u32 = 0x006d_c2f0;
/// `00401020` (no arguments: the memory manager) and `MemoryManager::GetThreadScrapHeap`
/// (engine map, `00aa42e0`; `thiscall`, no stack words).
const MEMORY_MANAGER_OF: u32 = 0x0040_1020;
const GET_THREAD_SCRAP_HEAP: u32 = 0x00aa_42e0;
/// `006b3eb0` (`thiscall`, two words: the array's initial setup).
const ARRAY_INITIALISE: u32 = 0x006b_3eb0;

// Translated from 006db680 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006db680 - the compiler's exception frame (FS:[0] chain, handler 00f05448) is not translated
/// Unnamed in the engine map; by its vtable the `BSScrapArray<float,1024>` constructor
/// (`thiscall`, three words: the two setup words and a heap; `RET 0xc`): the
/// `BSSimpleArray<float,1024>` base (`006dc2f0`), the vtable, the heap at +0x10 (the
/// thread's scrap heap from `MemoryManager::GetThreadScrapHeap` when `heap` is null), then
/// the setup `006b3eb0(first, second)`. Returns `this`.
pub fn fn_006db680(e: &mut Engine, this: Ptr, first: u32, second: u32, heap: u32) -> Ptr {
    e.call(SIMPLE_ARRAY_OF_FLOATS_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), SCRAP_ARRAY_OF_FLOATS_VTABLE);
    let heap = if heap != 0 {
        heap
    } else {
        let manager = e.call(MEMORY_MANAGER_OF, &args![]).u32();
        e.call(GET_THREAD_SCRAP_HEAP, &args![manager]).u32()
    };
    e.mem.set_u32(this.addr() + 0x10, heap);
    e.call(ARRAY_INITIALISE, &args![this, first, second]);
    this
}

// Translated from 006db740 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006db740 - the compiler's exception frame (FS:[0] chain, handler 00f05478) is not translated
/// Unnamed in the engine map; by its vtable the `BSScrapArray<float,1024>` destructor body
/// (`ECX` only): stores the vtable, runs `008454f0` on `this` with 1, then the
/// `BSSimpleArray<float,1024>` destructor body ([`fn_006db720`]).
pub fn fn_006db740(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), SCRAP_ARRAY_OF_FLOATS_VTABLE);
    e.call(ARRAY_008454F0, &args![this, 1u32]);
    fn_006db720(e, this);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006d_b720, fn_006db720(Ptr)),
        entry!(0x006d_b680, fn_006db680(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x006d_b740, fn_006db740(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sets_the_vtable_and_releases_with_one() {
        let mut e = Engine::new();
        e.register(ARRAY_008454F0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x10);
        e.call(0x006d_b720, &args![this]);
        assert_eq!(e.mem.u32(this), SIMPLE_ARRAY_OF_FLOATS_VTABLE);
        let calls: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == ARRAY_008454F0)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(calls, vec![vec![this, 1]]);
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    fn calls(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.register(SIMPLE_ARRAY_OF_FLOATS_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], SIMPLE_ARRAY_OF_FLOATS_VTABLE);
            Ret::default()
        });
        e.register(MEMORY_MANAGER_OF, |_, _| 0x0c00_0000u32.into_ret());
        e.register(GET_THREAD_SCRAP_HEAP, |_, _| 0x0c10_0000u32.into_ret());
        e.register(ARRAY_INITIALISE, |_, _| Ret::default());
        e.register(ARRAY_008454F0, |_, _| Ret::default());
        e
    }

    #[test]
    fn the_constructor_takes_the_given_heap_or_the_scrap_heap() {
        let mut e = rig();
        let this = e.mem.alloc(0x14);
        e.call_log = Some(vec![]);
        let back = e.call(0x006d_b680, &args![this, 4u32, 8u32, 0x0d00_0000u32]);
        assert_eq!(back.u32(), this);
        assert_eq!(e.mem.u32(this), SCRAP_ARRAY_OF_FLOATS_VTABLE);
        assert_eq!(e.mem.u32(this + 0x10), 0x0d00_0000);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, GET_THREAD_SCRAP_HEAP).is_empty());
        assert_eq!(calls(&log, ARRAY_INITIALISE), vec![vec![this, 4, 8]]);
        e.call_log = Some(vec![]);
        e.call(0x006d_b680, &args![this, 4u32, 8u32, 0u32]);
        assert_eq!(e.mem.u32(this + 0x10), 0x0c10_0000);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, GET_THREAD_SCRAP_HEAP), vec![vec![0x0c00_0000]]);
    }

    #[test]
    fn the_destructor_frees_then_runs_the_base_body() {
        let mut e = rig();
        let this = e.mem.alloc(0x14);
        e.call_log = Some(vec![]);
        e.call(0x006d_b740, &args![this]);
        // The base body leaves its own vtable.
        assert_eq!(e.mem.u32(this), SIMPLE_ARRAY_OF_FLOATS_VTABLE);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, ARRAY_008454F0),
            vec![vec![this, 1], vec![this, 1]]
        );
    }
}
