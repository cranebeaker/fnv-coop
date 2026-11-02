//! The C runtime's exit-table entry, small-block heap entry and new handler (MSVC 2008
//! LIBCMT, statically linked into FalloutNV.exe 1.4.0.525), translated from the exe:
//! `__onexit` (`00ec6553`) with the exit lock's two helpers (`00ecccc1`, `00ecccca`) and its
//! `__finally` block (`00ec6589`), `_V6_HeapAlloc` (`00ecd178`) with its `__finally` block
//! (`00ecd1be`), `__callnewh` (`00ede20b`), and the `__finally` block of the `eh vector
//! destructor iterator` (`00ec6019`).
//!
//! `_atexit` (`00ec658f`) and `__onexit_nolock` (`00ec6468`) are in `crt.rs`. Every call is made by address, as
//! the exe makes it, so a test can double each one.

use super::*;

/// `_lock` (`00eddf39`) and `_unlock` (`00edde5f`), cdecl: the lock number.
const LOCK_ADDR: u32 = 0x00ed_df39;
const UNLOCK_ADDR: u32 = 0x00ed_de5f;
/// The exit lock's helpers and `__onexit`'s `__finally` block (called by address).
const LOCK_EXIT: u32 = 0x00ec_ccc1;
const UNLOCK_EXIT: u32 = 0x00ec_ccca;
const ONEXIT_FINALLY: u32 = 0x00ec_6589;
/// `__onexit_nolock` (`00ec6468`, cdecl: the function).
const ONEXIT_NOLOCK: u32 = 0x00ec_6468;
/// `___sbh_alloc_block` (`00edea12`, cdecl: the size) and `_V6_HeapAlloc`'s `__finally` block.
const SBH_ALLOC_BLOCK: u32 = 0x00ed_ea12;
const HEAP_ALLOC_FINALLY: u32 = 0x00ec_d1be;
/// The small-block threshold `_V6_HeapAlloc` compares the size with (`01270938`; the CRT's
/// `__sbh_threshold`, a name not confirmed in Ghidra).
const SBH_THRESHOLD: u32 = 0x0127_0938;
/// The heap lock's number (`_HEAP_LOCK`, 4) and the exit lock's (`_EXIT_LOCK1`, 8).
const HEAP_LOCK: u32 = 4;
const EXIT_LOCK: u32 = 8;
/// `__decode_pointer` (`00ed0f92`, cdecl) and the encoded new handler `__callnewh` decodes
/// (`0127072c`; the CRT's `_pnhHeap`, a name not confirmed in Ghidra).
const DECODE_POINTER_ADDR: u32 = 0x00ed_0f92;
const NEW_HANDLER: u32 = 0x0127_072c;
/// `__ArrayUnwind` (`00ec5f70`, stdcall: array, element size, count, destructor).
const ARRAY_UNWIND: u32 = 0x00ec_5f70;

// Translated from 00ecccc1 (disassembled, FalloutNV.exe 1.4.0.525)
/// The exit lock taken (unnamed in the engine map; the CRT's `_lockexit`): `_lock(8)`.
pub fn lock_exit(e: &mut Engine) {
    e.call(LOCK_ADDR, &args![EXIT_LOCK]);
}

// Translated from 00ecccca (disassembled, FalloutNV.exe 1.4.0.525)
/// The exit lock given back (unnamed in the engine map; the CRT's `_unlockexit`):
/// `_unlock(8)`.
pub fn unlock_exit(e: &mut Engine) {
    e.call(UNLOCK_ADDR, &args![EXIT_LOCK]);
}

// Translated from 00ec6589 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__onexit`'s `__finally` block (unnamed; it uses nothing of the frame): the exit lock given
/// back (`00ecccca`).
pub fn onexit_finally(e: &mut Engine) {
    e.call(UNLOCK_EXIT, &args![]);
}

// Translated from 00ec6553 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ec6553 - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759, scope table 0117fcc8) is not translated; its __finally block (00ec6589) is called as the exe calls it on the way out.
/// `__onexit` (Ghidra's function ID), cdecl: the function to run at exit. With the exit lock
/// taken (`00ecccc1`), `__onexit_nolock(function)` answers; the `__finally` block gives the
/// lock back.
pub fn onexit(e: &mut Engine, function: u32) -> u32 {
    e.call(LOCK_EXIT, &args![]);
    let result = e.call(ONEXIT_NOLOCK, &args![function]).u32();
    e.call(ONEXIT_FINALLY, &args![]);
    result
}

// Translated from 00ecd1be (disassembled, FalloutNV.exe 1.4.0.525)
/// `_V6_HeapAlloc`'s `__finally` block (unnamed; it uses nothing of the frame): `_unlock(4)`.
pub fn heap_alloc_finally(e: &mut Engine) {
    e.call(UNLOCK_ADDR, &args![HEAP_LOCK]);
}

// Translated from 00ecd178 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ecd178 - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759, scope table 0117fef0) is not translated; its __finally block (00ecd1be) is called as the exe calls it on the way out.
/// `_V6_HeapAlloc` (Ghidra's function ID), cdecl: the size. A size above the small-block
/// threshold (`01270938`, an unsigned compare) answers null at once; otherwise, with the heap
/// lock taken (`_lock(4)`), `___sbh_alloc_block(size)` answers and the `__finally` block gives
/// the lock back.
pub fn v6_heap_alloc(e: &mut Engine, size: u32) -> u32 {
    let mut result = 0;
    if size <= e.global::<u32>(SBH_THRESHOLD) {
        e.call(LOCK_ADDR, &args![HEAP_LOCK]);
        result = e.call(SBH_ALLOC_BLOCK, &args![size]).u32();
        e.call(HEAP_ALLOC_FINALLY, &args![]);
    }
    result
}

// Translated from 00ede20b (disassembled, FalloutNV.exe 1.4.0.525)
/// `__callnewh` (Ghidra's function ID), cdecl: the size. The new handler (`0127072c`, decoded
/// with `__decode_pointer`) is called with the size (cdecl); answers 1 when there is one and it
/// answers nonzero, else 0.
pub fn callnewh(e: &mut Engine, size: u32) -> u32 {
    let encoded = e.global::<u32>(NEW_HANDLER);
    let handler = e.call(DECODE_POINTER_ADDR, &args![encoded]).u32();
    if handler != 0 && e.call(handler, &args![size]).u32() != 0 {
        return 1;
    }
    0
}

// Translated from 00ec6019 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__finally` block of the `eh vector destructor iterator` (`00ec5fce`), run on its frame:
/// the words it reads are the iterator's "done" local (`EBP-0x1c`, set to 1 once every
/// destructor ran) and its four stack arguments as the loop left them (the array cursor at
/// `EBP+8`, the element size, the count, the destructor), here in that order. When the loop did
/// not finish (a destructor raised an exception), `__ArrayUnwind(array, size, count,
/// destructor)` destroys the rest.
pub fn eh_vector_destructor_finally(
    e: &mut Engine,
    done: u32,
    array: u32,
    size: u32,
    count: u32,
    destructor: u32,
) {
    if done == 0 {
        e.call(ARRAY_UNWIND, &args![array, size, count, destructor]);
    }
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ec_ccc1, lock_exit()),
        entry!(0x00ec_ccca, unlock_exit()),
        entry!(0x00ec_6589, onexit_finally()),
        entry!(0x00ec_6553, onexit(u32) -> u32),
        entry!(0x00ec_d1be, heap_alloc_finally()),
        entry!(0x00ec_d178, v6_heap_alloc(u32) -> u32),
        entry!(0x00ed_e20b, callnewh(u32) -> u32),
        entry!(
            0x00ec_6019,
            eh_vector_destructor_finally(u32, u32, u32, u32, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The addresses of the calls made, in order, among `watched`.
    fn order(e: &Engine, watched: &[u32]) -> Vec<u32> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .map(|c| c.0)
            .filter(|a| watched.contains(a))
            .collect()
    }

    /// An engine with `_lock`/`_unlock` doubled to do nothing and the call log on.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(LOCK_ADDR, |_, _| Ret::default());
        e.register(UNLOCK_ADDR, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn onexit_runs_the_nolock_form_inside_the_exit_lock() {
        let mut e = engine();
        e.register(ONEXIT_NOLOCK, |_, a| (a[0] + 1).into_ret());
        assert_eq!(
            e.call(0x00ec_6553, &args![0x0090_0010u32]).u32(),
            0x0090_0011
        );
        assert_eq!(calls(&e, ONEXIT_NOLOCK), vec![vec![0x0090_0010]]);
        assert_eq!(calls(&e, LOCK_ADDR), vec![vec![8]]);
        assert_eq!(calls(&e, UNLOCK_ADDR), vec![vec![8]]);
        assert_eq!(
            order(&e, &[LOCK_ADDR, ONEXIT_NOLOCK, ONEXIT_FINALLY, UNLOCK_ADDR]),
            vec![LOCK_ADDR, ONEXIT_NOLOCK, ONEXIT_FINALLY, UNLOCK_ADDR]
        );
    }

    #[test]
    fn v6_heap_alloc_takes_small_blocks_under_the_heap_lock() {
        let mut e = engine();
        e.map(0x0127_0000, 0x1000);
        e.set_global::<u32>(SBH_THRESHOLD, 0x3f8);
        e.register(SBH_ALLOC_BLOCK, |_, a| (0x0a00_0000 + a[0]).into_ret());
        assert_eq!(e.call(0x00ec_d178, &args![0x3f8u32]).u32(), 0x0a00_03f8);
        assert_eq!(calls(&e, LOCK_ADDR), vec![vec![4]]);
        assert_eq!(
            order(
                &e,
                &[LOCK_ADDR, SBH_ALLOC_BLOCK, HEAP_ALLOC_FINALLY, UNLOCK_ADDR]
            ),
            vec![LOCK_ADDR, SBH_ALLOC_BLOCK, HEAP_ALLOC_FINALLY, UNLOCK_ADDR]
        );
        assert_eq!(calls(&e, UNLOCK_ADDR), vec![vec![4]]);
        // Above the threshold: null, no lock.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_d178, &args![0x3f9u32]).u32(), 0);
        assert!(calls(&e, LOCK_ADDR).is_empty());
        assert!(calls(&e, SBH_ALLOC_BLOCK).is_empty());
        // A failed block answers null after the unlock.
        e.register(SBH_ALLOC_BLOCK, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00ec_d178, &args![0x10u32]).u32(), 0);
        assert_eq!(calls(&e, UNLOCK_ADDR), vec![vec![4]]);
    }

    #[test]
    fn callnewh_calls_the_decoded_handler_with_the_size() {
        let mut e = engine();
        e.map(0x0127_0000, 0x1000);
        e.set_global::<u32>(NEW_HANDLER, 0x0b00_0000);
        // No handler: 0 without a call.
        e.register(DECODE_POINTER_ADDR, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00ed_e20b, &args![0x20u32]).u32(), 0);
        assert_eq!(calls(&e, DECODE_POINTER_ADDR), vec![vec![0x0b00_0000]]);
        e.register(DECODE_POINTER_ADDR, |_, a| (a[0] + 0x10).into_ret());
        e.register(0x0b00_0010, |_, a| u32::from(a[0] == 0x20).into_ret());
        assert_eq!(e.call(0x00ed_e20b, &args![0x20u32]).u32(), 1);
        assert_eq!(e.call(0x00ed_e20b, &args![0x30u32]).u32(), 0);
        assert_eq!(calls(&e, 0x0b00_0010), vec![vec![0x20], vec![0x30]]);
    }

    #[test]
    fn the_eh_vector_finally_block_unwinds_only_an_unfinished_loop() {
        let mut e = engine();
        e.register(ARRAY_UNWIND, |_, _| Ret::default());
        e.call(
            0x00ec_6019,
            &args![1u32, 0x100u32, 8u32, 0u32, 0x0040_1000u32],
        );
        assert!(calls(&e, ARRAY_UNWIND).is_empty());
        e.call(
            0x00ec_6019,
            &args![0u32, 0x110u32, 8u32, 2u32, 0x0040_1000u32],
        );
        assert_eq!(
            calls(&e, ARRAY_UNWIND),
            vec![vec![0x110, 8, 2, 0x0040_1000]]
        );
    }

    #[test]
    fn the_exit_lock_helpers_and_finally_blocks_take_and_give_their_locks() {
        let mut e = engine();
        e.call(0x00ec_ccc1, &args![]);
        e.call(0x00ec_ccca, &args![]);
        e.call(0x00ec_6589, &args![]);
        e.call(0x00ec_d1be, &args![]);
        assert_eq!(calls(&e, LOCK_ADDR), vec![vec![8]]);
        assert_eq!(calls(&e, UNLOCK_ADDR), vec![vec![8], vec![8], vec![4]]);
    }
}
