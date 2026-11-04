//! `fallout/misc/appmemory.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The memory manager's pool table (`00866770`), the routine that frees one
//! level of memory (`00866a90`, called by the scope in `run_00877fc0.rs`),
//! the lazily built scrap-heap singleton (`00866d10`) and three thin
//! accessors that forward to the element of an array at `+0x50` of the
//! object they are called on.

#[allow(unused_imports)]
use crate::prelude::*;

/// `00aa3cc0` (`__thiscall` on the manager, three stack words: the pool's
/// block size, its limit in bytes, and the address of its name string).
const SPECIFY_POOL: u32 = 0x00aa_3cc0;

/// The pools `00866770` specifies, in call order: block size, limit in bytes,
/// address of the name string (`"Pool 596"` ... `"Pool 12"`).
const POOLS: [(u32, u32, u32); 37] = [
    (0x8, 0x60_0000, 0x0108_24dc),
    (0xc, 0x50_0000, 0x0108_24d4),
    (0x10, 0x40_0000, 0x0108_24cc),
    (0x14, 0x40_0000, 0x0108_24c4),
    (0x18, 0x80_0000, 0x0108_24bc),
    (0x1c, 0x40_0000, 0x0108_24b4),
    (0x20, 0x80_0000, 0x0108_24ac),
    (0x24, 0x30_0000, 0x0108_24a4),
    (0x28, 0x20_0000, 0x0108_249c),
    (0x2c, 0x40_0000, 0x0108_2494),
    (0x30, 0x20_0000, 0x0108_248c),
    (0x34, 0x30_0000, 0x0108_2484),
    (0x38, 0x10_0000, 0x0108_247c),
    (0x40, 0x20_0000, 0x0108_2474),
    (0x48, 0xd0_0000, 0x0108_246c),
    (0x4c, 0x10_0000, 0x0108_2464),
    (0x50, 0x30_0000, 0x0108_245c),
    (0x5c, 0x20_0000, 0x0108_2454),
    (0x60, 0xf0_0000, 0x0108_244c),
    (0x64, 0x10_0000, 0x0108_2440),
    (0x68, 0x40_0000, 0x0108_2434),
    (0x78, 0x40_0000, 0x0108_2428),
    (0x80, 0x10_0000, 0x0108_241c),
    (0xa0, 0x20_0000, 0x0108_2410),
    (0xc0, 0x20_0000, 0x0108_2404),
    (0xc8, 0x30_0000, 0x0108_23f8),
    (0xd0, 0x30_0000, 0x0108_23ec),
    (0xf0, 0x30_0000, 0x0108_23e0),
    (0x100, 0x10_0000, 0x0108_23d4),
    (0x110, 0x20_0000, 0x0108_23c8),
    (0x120, 0x30_0000, 0x0108_23bc),
    (0x190, 0x10_0000, 0x0108_23b0),
    (0x1c0, 0x10_0000, 0x0108_23a4),
    (0x1e0, 0x40_0000, 0x0108_2398),
    (0x1fc, 0x40_0000, 0x0108_238c),
    (0x240, 0x20_0000, 0x0108_2380),
    (0x254, 0x10_0000, 0x0108_2374),
];

/// Counter of the "free more memory" rounds in progress (`011de70c`);
/// level 0 clears it and level 7/8 count it up to `MAX_ROUNDS`.
const ROUNDS: u32 = 0x011d_e70c;
/// A round counter at or above this ends the rounds.
const MAX_ROUNDS: u32 = 0x3a98;
/// The object `0044edb0` is called on (a global pointer).
const COMPARE_OBJECT: u32 = 0x011d_ea0c;
/// The object most level handlers call methods on (a global pointer).
const LEVEL_OBJECT: u32 = 0x011d_ea10;
/// The global pointer whose object `00866da0`/`00866dc0`/`00866de0` are
/// called on in levels 7 and 8.
const ARRAY_OWNER: u32 = 0x0120_2d98;
/// The object level 4 and level 5 test with `0078d200`.
const QUERY_OBJECT: u32 = 0x011f_11a0;
/// The scrap heap manager singleton (`ScrapHeapManager::Instance`).
const SCRAP_HEAP_MANAGER: u32 = 0x011f_63b4;
/// Size of a `ScrapHeapManager` (`00aa5860` constructs it in place).
const SCRAP_HEAP_MANAGER_SIZE: u32 = 0x240;
/// `00401000`: the allocator (`MemoryManager::Allocate`) taking a size.
const ALLOCATE: u32 = 0x0040_1000;
/// The memory manager's word at `+0x134` that level 7/8 sets to 6.
const MANAGER_STATE: u32 = 0x134;

// Translated from 00866770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MemoryManager::SpecifyPools` (Xbox PDB): registers the 37 pools of the
/// pool allocator, one `00aa3cc0` call each (block size, limit in bytes,
/// name string), in the order of `POOLS`.
pub fn memory_manager_specify_pools(e: &mut Engine, this: Ptr) {
    for (block_size, limit, name) in POOLS {
        e.call(SPECIFY_POOL, &args![this, block_size, limit, name]);
    }
}

/// The first call every level handler makes, then compared with
/// `0044edb0` on the object at `011dea0c`: whether the two agree.
fn same_as_compare_object(e: &mut Engine) -> bool {
    let first = e.call(0x0040_fc90, &[]).u32();
    let compare_object = e.global::<u32>(COMPARE_OBJECT);
    let second = e.call(0x0044_edb0, &args![compare_object]).u32();
    first == second
}

/// Calls `00866da0` on the owner's element, then when it equals the result
/// of `0040fc90` clears it with `00866dc0` and `00866de0`: the body of one
/// of the two blocks of levels 7 and 8. Each of the three calls loads the
/// owner from `01202d98` again (`00866c49`, `00866c61`, `00866c6e` for
/// element 0; `00866c7b`, `00866c93`, `00866ca0` for element 1).
fn reset_array_element(e: &mut Engine, index: u32) {
    let owner = e.global::<u32>(ARRAY_OWNER);
    let current = fn_00866da0(e, Ptr::new(owner), index);
    let expected = e.call(0x0040_fc90, &[]).u32();
    if current == expected {
        let owner = e.global::<u32>(ARRAY_OWNER);
        fn_00866dc0(e, Ptr::new(owner), index);
        let owner = e.global::<u32>(ARRAY_OWNER);
        fn_00866de0(e, Ptr::new(owner), index);
    }
}

/// The part of levels 7 and 8 that runs when `0040fc90` and `0044edb0` do
/// not agree: sets the manager's state word to 6 and, until `MAX_ROUNDS`
/// rounds have run, resets both array elements and takes one round
/// (returning the decremented level); afterwards it sets the flag.
fn unsynchronized_round(e: &mut Engine, this: Ptr, level: u32, flag: Ptr) -> u32 {
    e.mem.set_u32(this.addr().wrapping_add(MANAGER_STATE), 6);
    if e.global::<u32>(ROUNDS) < MAX_ROUNDS {
        reset_array_element(e, 0);
        reset_array_element(e, 1);
        e.call(0x0040_fca0, &args![1u32]);
        let rounds = e.global::<u32>(ROUNDS);
        e.set_global(ROUNDS, rounds.wrapping_add(1));
        level.wrapping_sub(1)
    } else {
        if e.mem.u8(flag.addr()) == 0 {
            e.mem.set_u8(flag.addr(), 1);
        }
        level
    }
}

/// The end of levels 4 and 5 (`with_query`) and all of level 3: when asked,
/// first the `0078d200` test on the object at `011f11a0`; then `00c459d0(1)`.
fn level_four_tail(e: &mut Engine, with_query: bool) {
    if with_query {
        let query = e.call(0x0078_d200, &args![QUERY_OBJECT]).u8();
        if query != 0 {
            e.call(0x0086_8d70, &args![1u32]);
            e.call(0x0040_fba0, &args![QUERY_OBJECT]);
        }
    }
    e.call(0x00c4_59d0, &args![1u32]);
}

// Translated from 00866a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees one level of memory (no name in the Xbox PDB; the scope in
/// `run_00877fc0.rs` calls it as "free level"). `__thiscall` on the memory
/// manager with three stack words: the first is never read, then the level
/// (0 to 8) and a pointer to a flag byte, which is set to 1 on entry. The
/// returned value is the level plus one, after levels 5, 7 and 8 may have
/// decremented it (that makes the caller ask again for the same level).
///
/// The handlers, each a jump table case: 0 resets the round counter and
/// has the object at `011dea10` (`00452490(0)`) do something when
/// `0040fc90` and `0044edb0` agree; 1 runs the scrap heap manager's
/// `00aa5c80`; 2 runs a chain of checks on the object `00652110` returns;
/// 3 only calls `00c459d0(1)`; 5 prepares (when they agree) and continues
/// into 4, which asks `0078d200`; 6 calls `00aa7030` when they agree; 7 and
/// 8 take rounds (`unsynchronized_round`) when they do not agree.
pub fn fn_00866a90(e: &mut Engine, this: Ptr, _unused_0: u32, mut level: u32, flag: Ptr) -> u32 {
    e.mem.set_u8(flag.addr(), 1);
    let same = same_as_compare_object(e);
    match level {
        0 => {
            e.set_global(ROUNDS, 0u32);
            if same {
                let object = e.global::<u32>(LEVEL_OBJECT);
                e.call(0x0045_2490, &args![object, 0u32]);
            }
        }
        1 => {
            let scrap_heap = scrap_heap_manager_instance(e);
            e.call(0x00aa_5c80, &args![scrap_heap]);
        }
        2 => {
            let object = e.call(0x0065_2110, &[]).u32();
            if object != 0 {
                let object = e.call(0x0065_2110, &[]).u32();
                let ready = e.call(0x0052_9ea0, &args![object]).u8();
                if ready != 0 {
                    let pair_begin = e.call(0x0065_2160, &[]).u8();
                    if pair_begin != 0 {
                        let state = e.call(0x0065_2130, &[]).u32();
                        if state == 1 {
                            let object = e.call(0x0065_2110, &[]).u32();
                            e.call(0x0065_0a30, &args![object, 0u32]);
                        }
                        e.call(0x0065_2190, &[]);
                    }
                }
            }
        }
        3 => level_four_tail(e, false),
        4 => level_four_tail(e, true),
        5 => {
            if same {
                e.call(0x0086_9190, &args![0u32]);
                // 00866b67, 00866b88, 00866b95: each call loads the object
                // from `011dea10` again.
                let object = e.global::<u32>(LEVEL_OBJECT);
                if e.call(0x0045_3a80, &args![object]).u8() != 0 {
                    level = level.wrapping_sub(1);
                } else {
                    let object = e.global::<u32>(LEVEL_OBJECT);
                    e.call(0x0045_39a0, &args![object, 1u32, 0u32]);
                }
                let object = e.global::<u32>(LEVEL_OBJECT);
                e.call(0x0045_2490, &args![object, 0u32]);
                e.call(0x0086_9190, &args![1u32]);
                e.call(0x0086_8d70, &args![1u32]);
                level_four_tail(e, true);
            }
        }
        6 => {
            if same {
                e.call(0x00aa_7030, &[]);
            }
        }
        7 => {
            if e.mem.u8(flag.addr()) == 0 && same {
                e.mem.set_u8(flag.addr(), 1);
            } else if !same {
                level = unsynchronized_round(e, this, level, flag);
            }
        }
        8 if !same => {
            level = unsynchronized_round(e, this, level, flag);
        }
        _ => {}
    }
    level.wrapping_add(1)
}

// Translated from 00866d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ScrapHeapManager::Instance` (Xbox PDB): the scrap heap manager
/// singleton at `011f63b4`, built on first use (a 0x240-byte block from
/// the allocator, constructed in place by `00aa5860`).
// Translation incomplete: 00866d10 - the compiler's exception-unwind frame (FS:[0]) is not translated.
pub fn scrap_heap_manager_instance(e: &mut Engine) -> Ptr {
    if e.global::<u32>(SCRAP_HEAP_MANAGER) == 0 {
        let block = e.call(ALLOCATE, &args![SCRAP_HEAP_MANAGER_SIZE]).u32();
        let manager = if block != 0 {
            e.call(0x00aa_5860, &args![block]).u32()
        } else {
            0
        };
        e.set_global(SCRAP_HEAP_MANAGER, manager);
    }
    Ptr::new(e.global::<u32>(SCRAP_HEAP_MANAGER))
}

/// The element `index` of the array whose pointer is at `+0x50` of `this`.
fn array_element(e: &Engine, this: Ptr, index: u32) -> u32 {
    let array = e.mem.u32(this.addr().wrapping_add(0x50));
    e.mem.u32(array.wrapping_add(index.wrapping_mul(4)))
}

// Translated from 00866da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `0044ddc0` on element `index` of the array at `+0x50` and returns
/// its result.
pub fn fn_00866da0(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let element = array_element(e, this, index);
    e.call(0x0044_ddc0, &args![element]).u32()
}

// Translated from 00866dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `00442530` on element `index` of the array at `+0x50` and returns
/// its result.
pub fn fn_00866dc0(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let element = array_element(e, this, index);
    e.call(0x0044_2530, &args![element]).u32()
}

// Translated from 00866de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `004424c0` on element `index` of the array at `+0x50` and returns
/// its result.
pub fn fn_00866de0(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let element = array_element(e, this, index);
    e.call(0x0044_24c0, &args![element]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00866770, memory_manager_specify_pools(Ptr)),
        entry!(0x00866a90, fn_00866a90(Ptr, u32, u32, Ptr) -> u32),
        entry!(0x00866d10, scrap_heap_manager_instance() -> Ptr),
        entry!(0x00866da0, fn_00866da0(Ptr, u32) -> u32),
        entry!(0x00866dc0, fn_00866dc0(Ptr, u32) -> u32),
        entry!(0x00866de0, fn_00866de0(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word_ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// Every address the level handlers call, as a plain double returning
    /// zero; the tests then override what they need.
    const LEVEL_CALLEES: [u32; 22] = [
        0x0040_fc90,
        0x0044_edb0,
        0x0045_2490,
        0x0045_3a80,
        0x0045_39a0,
        0x0052_9ea0,
        0x0065_0a30,
        0x0065_2110,
        0x0065_2130,
        0x0065_2160,
        0x0065_2190,
        0x0078_d200,
        0x0086_8d70,
        0x0086_9190,
        0x00aa_5c80,
        0x00aa_7030,
        0x00c4_59d0,
        0x0040_fba0,
        0x0040_fca0,
        0x0044_ddc0,
        0x0044_2530,
        0x0044_24c0,
    ];

    /// An engine whose level callees are zero-returning doubles and whose
    /// two comparison calls agree (`0040fc90` and `0044edb0` both return 5).
    fn level_engine() -> (Engine, Ptr, Ptr) {
        let mut e = Engine::new();
        for addr in LEVEL_CALLEES {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(0x0040_fc90, |_, _| word_ret(5));
        e.register(0x0044_edb0, |_, _| word_ret(5));
        e.map(ROUNDS, 4);
        e.map(COMPARE_OBJECT, 4);
        e.map(LEVEL_OBJECT, 4);
        e.map(ARRAY_OWNER, 4);
        e.map(QUERY_OBJECT, 4);
        e.set_global(COMPARE_OBJECT, 0x0aaa_0001u32);
        e.set_global(LEVEL_OBJECT, 0x0aaa_0002u32);
        let manager = Ptr::new(e.mem.alloc(0x140));
        let flag = Ptr::new(e.mem.alloc(4));
        (e, manager, flag)
    }

    /// Runs the level; returns the result and the calls it made in order.
    fn run_level(
        e: &mut Engine,
        manager: Ptr,
        flag: Ptr,
        level: u32,
    ) -> (u32, Vec<(u32, Vec<u32>)>) {
        e.call_log = Some(vec![]);
        let result = e
            .call(0x0086_6a90, &args![manager, 0u32, level, flag])
            .u32();
        let mut log = e.call_log.take().unwrap();
        log.remove(0);
        (result, log)
    }

    #[test]
    fn pools_are_specified_in_order() {
        let mut e = Engine::new();
        e.register(SPECIFY_POOL, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0086_6770, &[0x1000]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 1 + 37);
        assert_eq!(
            log[1],
            (SPECIFY_POOL, vec![0x1000, 8, 0x60_0000, 0x0108_24dc])
        );
        assert_eq!(
            log[37],
            (SPECIFY_POOL, vec![0x1000, 0x254, 0x10_0000, 0x0108_2374])
        );
    }

    #[test]
    fn level_zero_resets_the_counter_and_acts_when_the_calls_agree() {
        let (mut e, manager, flag) = level_engine();
        e.set_global(ROUNDS, 77u32);
        let (result, log) = run_level(&mut e, manager, flag, 0);
        assert_eq!(result, 1);
        assert_eq!(e.global::<u32>(ROUNDS), 0);
        assert_eq!(e.mem.u8(flag.addr()), 1);
        assert_eq!(log.last().unwrap(), &(0x0045_2490, vec![0x0aaa_0002, 0]));
        // Disagreement: nothing is called after the comparison.
        e.register(0x0044_edb0, |_, _| word_ret(6));
        let (_, log) = run_level(&mut e, manager, flag, 0);
        assert_eq!(log.len(), 2);
    }

    #[test]
    fn level_one_runs_the_scrap_heap_manager() {
        let (mut e, manager, flag) = level_engine();
        e.map(SCRAP_HEAP_MANAGER, 4);
        e.set_global(SCRAP_HEAP_MANAGER, 0x0bbb_0000u32);
        let (result, log) = run_level(&mut e, manager, flag, 1);
        assert_eq!(result, 2);
        assert_eq!(log.last().unwrap(), &(0x00aa_5c80, vec![0x0bbb_0000]));
    }

    #[test]
    fn level_two_checks_in_order_and_only_acts_on_state_one() {
        let (mut e, manager, flag) = level_engine();
        // No object: only the first lookup.
        let (_, log) = run_level(&mut e, manager, flag, 2);
        assert_eq!(log.last().unwrap().0, 0x0065_2110);
        let before = log.len();
        e.register(0x0065_2110, |_, _| word_ret(0x0ccc_0000));
        e.register(0x0052_9ea0, |_, _| word_ret(1));
        e.register(0x0065_2160, |_, _| word_ret(1));
        e.register(0x0065_2130, |_, _| word_ret(1));
        let (_, log) = run_level(&mut e, manager, flag, 2);
        assert!(log.len() > before);
        let addrs: Vec<u32> = log.iter().map(|c| c.0).collect();
        assert_eq!(
            &addrs[addrs.len() - 4..],
            &[0x0065_2130, 0x0065_2110, 0x0065_0a30, 0x0065_2190]
        );
        assert_eq!(log[log.len() - 2].1, vec![0x0ccc_0000, 0]);
        // A different state skips the call at 00650a30 but still ends the pair.
        e.register(0x0065_2130, |_, _| word_ret(2));
        let (_, log) = run_level(&mut e, manager, flag, 2);
        assert_eq!(log.last().unwrap().0, 0x0065_2190);
        assert!(!log.iter().any(|c| c.0 == 0x0065_0a30));
    }

    #[test]
    fn levels_three_and_four_call_the_tail() {
        let (mut e, manager, flag) = level_engine();
        let (result, log) = run_level(&mut e, manager, flag, 3);
        assert_eq!(result, 4);
        assert_eq!(log.last().unwrap(), &(0x00c4_59d0, vec![1]));
        assert!(!log.iter().any(|c| c.0 == 0x0078_d200));
        let (_, log) = run_level(&mut e, manager, flag, 4);
        assert!(log.iter().any(|c| c.0 == 0x0078_d200));
        assert!(!log.iter().any(|c| c.0 == 0x0040_fba0));
        e.register(0x0078_d200, |_, _| word_ret(1));
        let (_, log) = run_level(&mut e, manager, flag, 4);
        let addrs: Vec<u32> = log.iter().map(|c| c.0).collect();
        assert_eq!(
            &addrs[addrs.len() - 3..],
            &[0x0086_8d70, 0x0040_fba0, 0x00c4_59d0]
        );
    }

    #[test]
    fn level_five_decrements_when_the_object_reports_and_continues_into_four() {
        let (mut e, manager, flag) = level_engine();
        e.register(0x0045_3a80, |_, _| word_ret(1));
        let (result, log) = run_level(&mut e, manager, flag, 5);
        assert_eq!(result, 5);
        assert!(!log.iter().any(|c| c.0 == 0x0045_39a0));
        assert_eq!(log.last().unwrap().0, 0x00c4_59d0);
        e.register(0x0045_3a80, |_, _| word_ret(0));
        let (result, log) = run_level(&mut e, manager, flag, 5);
        assert_eq!(result, 6);
        assert!(log.contains(&(0x0045_39a0, vec![0x0aaa_0002, 1, 0])));
        // Disagreement: nothing happens.
        e.register(0x0044_edb0, |_, _| word_ret(9));
        let (result, log) = run_level(&mut e, manager, flag, 5);
        assert_eq!(result, 6);
        assert_eq!(log.len(), 2);
    }

    #[test]
    fn level_six_calls_only_when_the_calls_agree() {
        let (mut e, manager, flag) = level_engine();
        let (_, log) = run_level(&mut e, manager, flag, 6);
        assert_eq!(log.last().unwrap().0, 0x00aa_7030);
        e.register(0x0044_edb0, |_, _| word_ret(9));
        let (result, log) = run_level(&mut e, manager, flag, 6);
        assert_eq!(result, 7);
        assert_eq!(log.last().unwrap().0, 0x0044_edb0);
    }

    #[test]
    fn levels_seven_and_eight_take_rounds_until_the_limit() {
        let (mut e, manager, flag) = level_engine();
        e.register(0x0044_edb0, |_, _| word_ret(9));
        e.map(0x0bcd_0000, 0x100);
        e.mem.set_u32(0x0bcd_0050, 0x0bce_0000);
        e.map(0x0bce_0000, 8);
        e.set_global(ARRAY_OWNER, 0x0bcd_0000u32);
        let (result, log) = run_level(&mut e, manager, flag, 8);
        // One round: the level is taken down by one, the counter goes up.
        assert_eq!(result, 8);
        assert_eq!(e.global::<u32>(ROUNDS), 1);
        assert_eq!(e.mem.u32(manager.addr() + 0x134), 6);
        assert_eq!(log.last().unwrap(), &(0x0040_fca0, vec![1]));
        // The array elements equal 0 and the call returns 0: both reset.
        e.register(0x0040_fc90, |_, _| word_ret(0));
        e.register(0x0044_ddc0, |_, _| word_ret(0));
        let (_, log) = run_level(&mut e, manager, flag, 7);
        assert_eq!(log.iter().filter(|c| c.0 == 0x0044_2530).count(), 2);
        assert_eq!(log.iter().filter(|c| c.0 == 0x0044_24c0).count(), 2);
        // At the limit the flag is set instead.
        e.set_global(ROUNDS, MAX_ROUNDS);
        let (result, _) = run_level(&mut e, manager, flag, 8);
        assert_eq!(result, 9);
        assert_eq!(e.global::<u32>(ROUNDS), MAX_ROUNDS);
    }

    #[test]
    fn levels_load_the_global_objects_again_for_every_call() {
        // Level 5 (00866b67, 00866b88, 00866b95): `011dea10` is read before
        // each of the three calls on it.
        let (mut e, manager, flag) = level_engine();
        e.register(0x0045_3a80, |e, _| {
            e.set_global(LEVEL_OBJECT, 0x0aaa_0003u32);
            word_ret(0)
        });
        e.register(0x0045_39a0, |e, _| {
            e.set_global(LEVEL_OBJECT, 0x0aaa_0004u32);
            word_ret(0)
        });
        let (_, log) = run_level(&mut e, manager, flag, 5);
        let on_object: Vec<(u32, Vec<u32>)> = log
            .into_iter()
            .filter(|c| [0x0045_3a80, 0x0045_39a0, 0x0045_2490].contains(&c.0))
            .collect();
        assert_eq!(
            on_object,
            vec![
                (0x0045_3a80, vec![0x0aaa_0002]),
                (0x0045_39a0, vec![0x0aaa_0003, 1, 0]),
                (0x0045_2490, vec![0x0aaa_0004, 0]),
            ]
        );
        // Levels 7/8 (00866c49 ... 00866ca0): `01202d98` is read before each
        // of the six element calls.
        let (mut e, manager, flag) = level_engine();
        e.register(0x0040_fc90, |_, _| word_ret(0));
        e.register(0x0044_edb0, |_, _| word_ret(9));
        for (owner, array, first) in [
            (0x0bcd_0000u32, 0x0bce_0000u32, 0xa0u32),
            (0x0bdd_0000, 0x0bde_0000, 0xb0),
        ] {
            e.map(owner, 0x100);
            e.mem.set_u32(owner + 0x50, array);
            e.map(array, 8);
            e.mem.set_u32(array, first);
            e.mem.set_u32(array + 4, first + 1);
        }
        e.set_global(ARRAY_OWNER, 0x0bcd_0000u32);
        e.register(0x0044_ddc0, |e, _| {
            e.set_global(ARRAY_OWNER, 0x0bdd_0000u32);
            word_ret(0)
        });
        e.register(0x0044_2530, |e, _| {
            e.set_global(ARRAY_OWNER, 0x0bcd_0000u32);
            word_ret(0)
        });
        let (result, log) = run_level(&mut e, manager, flag, 8);
        assert_eq!(result, 8);
        let on_elements: Vec<(u32, Vec<u32>)> = log
            .into_iter()
            .filter(|c| [0x0044_ddc0, 0x0044_2530, 0x0044_24c0].contains(&c.0))
            .collect();
        assert_eq!(
            on_elements,
            vec![
                (0x0044_ddc0, vec![0xa0]),
                (0x0044_2530, vec![0xb0]),
                (0x0044_24c0, vec![0xa0]),
                (0x0044_ddc0, vec![0xa1]),
                (0x0044_2530, vec![0xb1]),
                (0x0044_24c0, vec![0xa1]),
            ]
        );
    }

    #[test]
    fn level_seven_sets_the_flag_when_clear_and_the_calls_agree() {
        let (mut e, manager, flag) = level_engine();
        // The flag is always written 1 on entry, so the first branch of
        // level 7 cannot be taken from a clean call: agreeing calls do nothing.
        let (result, log) = run_level(&mut e, manager, flag, 7);
        assert_eq!(result, 8);
        assert_eq!(log.len(), 2);
    }

    #[test]
    fn levels_past_eight_only_return_the_next_number() {
        let (mut e, manager, flag) = level_engine();
        let (result, log) = run_level(&mut e, manager, flag, 20);
        assert_eq!(result, 21);
        assert_eq!(log.len(), 2);
    }

    #[test]
    fn scrap_heap_manager_is_built_once() {
        let mut e = Engine::new();
        e.map(SCRAP_HEAP_MANAGER, 4);
        e.register(ALLOCATE, |_, _| word_ret(0x0ddd_0000));
        e.register(0x00aa_5860, |_, a| word_ret(a[0] + 0x10));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0086_6d10, &[]).u32(), 0x0ddd_0010);
        assert_eq!(e.call(0x0086_6d10, &[]).u32(), 0x0ddd_0010);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.iter().filter(|c| c.0 == ALLOCATE).count(), 1);
        assert_eq!(log[1], (ALLOCATE, vec![0x240]));
        assert_eq!(e.global::<u32>(SCRAP_HEAP_MANAGER), 0x0ddd_0010);
    }

    #[test]
    fn scrap_heap_manager_is_null_when_the_allocation_fails() {
        let mut e = Engine::new();
        e.map(SCRAP_HEAP_MANAGER, 4);
        e.register(ALLOCATE, |_, _| word_ret(0));
        assert_eq!(e.call(0x0086_6d10, &[]).u32(), 0);
    }

    #[test]
    fn array_accessors_forward_the_element() {
        let mut e = Engine::new();
        let owner = e.mem.alloc(0x60);
        let array = e.mem.alloc(8);
        e.mem.set_u32(owner + 0x50, array);
        e.mem.set_u32(array, 0x1111);
        e.mem.set_u32(array + 4, 0x2222);
        e.register(0x0044_ddc0, |_, a| word_ret(a[0] + 1));
        e.register(0x0044_2530, |_, a| word_ret(a[0] + 2));
        e.register(0x0044_24c0, |_, a| word_ret(a[0] + 3));
        assert_eq!(e.call(0x0086_6da0, &[owner, 1]).u32(), 0x2223);
        assert_eq!(e.call(0x0086_6dc0, &[owner, 0]).u32(), 0x1113);
        assert_eq!(e.call(0x0086_6de0, &[owner, 1]).u32(), 0x2225);
    }
}
