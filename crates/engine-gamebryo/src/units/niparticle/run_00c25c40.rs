//! `NiParticle/run_00c25c40` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The critical section guarding the update-task pool (`01202600`) and the
/// imported `EnterCriticalSection` / `LeaveCriticalSection` (import slots).
const POOL_LOCK: u32 = 0x0120_2600;
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// The update-task pool (`011aeb98`): its free list's array (+0), count
/// (+8, `011aeba0`), and the number of tasks the next refill makes
/// (`011aeba8`).
const POOL: u32 = 0x011a_eb98;
const POOL_FREE_COUNT: u32 = 0x011a_eba0;
const POOL_REFILL_SIZE: u32 = 0x011a_eba8;
/// `00c25b30` (unnamed, `NiParticle/nipsysupdatetask.cpp`, `thiscall` on
/// the pool, one word: the number of tasks): refills the pool.
const POOL_REFILL: u32 = 0x00c2_5b30;
/// `00aa13e0` (`cdecl`, one word: the size): the Ni allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `00c24c90` (the `NiPSysData` constructor, `thiscall`, no words).
const PSYS_DATA_CONSTRUCTOR: u32 = 0x00c2_4c90;
/// The `BSStripPSysData` vtable (RTTI `.?AVBSStripPSysData@@`).
const STRIP_PSYS_DATA_VTABLE: u32 = 0x010b_e514;

// Translated from 00c25c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, no words): takes an update task from
/// the pool under its lock. An empty free list is refilled first
/// (`00c25b30` with the refill size, which then doubles). The answer is the
/// list's first entry; the count drops by one and the last entry moves to
/// the front.
pub fn fn_00c25c40(e: &mut Engine) -> u32 {
    e.call(ENTER_CRITICAL_SECTION, &args![POOL_LOCK]);
    if e.mem.u32(POOL_FREE_COUNT) == 0 {
        let size = e.mem.u32(POOL_REFILL_SIZE);
        e.call(POOL_REFILL, &args![POOL, size]);
        let size = e.mem.u32(POOL_REFILL_SIZE);
        e.mem.set_u32(POOL_REFILL_SIZE, size.wrapping_add(size));
    }
    let count = e.mem.u32(POOL_FREE_COUNT).wrapping_sub(1);
    let array = e.mem.u32(POOL);
    let first = e.mem.u32(array);
    e.mem.set_u32(POOL_FREE_COUNT, count);
    let last = e.mem.u32(array.wrapping_add(count.wrapping_mul(4)));
    e.mem.set_u32(array, last);
    e.call(LEAVE_CRITICAL_SECTION, &args![POOL_LOCK]);
    first
}

// Translated from 00c25ca0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The `BSStripPSysData` creation function (Ghidra made no function of it;
/// `cdecl`, no words): 0x90 bytes from the Ni allocator, constructed as an
/// `NiPSysData` (`00c24c90`), given the `BSStripPSysData` vtable and the
/// word at +0x7C cleared; null when the allocation fails.
pub fn fn_00c25ca0(e: &mut Engine) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![0x90u32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    e.call(PSYS_DATA_CONSTRUCTOR, &args![block]);
    e.mem.set_u32(block, STRIP_PSYS_DATA_VTABLE);
    e.mem.set_u32(block + 0x7c, 0);
    Ptr::new(block)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c2_5c40, fn_00c25c40() -> u32),
        entry!(0x00c2_5ca0, fn_00c25ca0() -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pool(e: &mut Engine, entries: &[u32]) {
        e.map(0x011a_e000, 0x1000);
        let array = e.mem.alloc(0x20);
        for (i, entry) in entries.iter().enumerate() {
            e.mem.set_u32(array + 4 * i as u32, *entry);
        }
        e.set_global(POOL, array);
        e.set_global(POOL_FREE_COUNT, entries.len() as u32);
        e.set_global(POOL_REFILL_SIZE, 4u32);
        e.register(ENTER_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(LEAVE_CRITICAL_SECTION, |_, _| Ret::default());
    }

    #[test]
    fn fn_00c25c40_takes_the_first_task_and_moves_the_last_forward() {
        let mut e = Engine::new();
        pool(&mut e, &[0xa1, 0xa2, 0xa3]);
        e.register(POOL_REFILL, |_, _| panic!("no refill"));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c2_5c40, &args![]).u32(), 0xa1);
        let array = e.mem.u32(POOL);
        assert_eq!(e.mem.u32(array), 0xa3);
        assert_eq!(e.mem.u32(POOL_FREE_COUNT), 2);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (ENTER_CRITICAL_SECTION, vec![POOL_LOCK]));
        assert_eq!(log[2], (LEAVE_CRITICAL_SECTION, vec![POOL_LOCK]));
    }

    #[test]
    fn fn_00c25c40_refills_an_empty_pool_and_doubles_the_refill() {
        let mut e = Engine::new();
        pool(&mut e, &[]);
        e.register(POOL_REFILL, |e, a| {
            assert_eq!(a, [POOL, 4]);
            let array = e.mem.u32(POOL);
            e.mem.set_u32(array, 0xb1);
            e.mem.set_u32(array + 4, 0xb2);
            e.mem.set_u32(POOL_FREE_COUNT, 2);
            Ret::default()
        });
        assert_eq!(e.call(0x00c2_5c40, &args![]).u32(), 0xb1);
        assert_eq!(e.mem.u32(POOL_REFILL_SIZE), 8);
        assert_eq!(e.mem.u32(POOL_FREE_COUNT), 1);
        assert_eq!(e.mem.u32(e.mem.u32(POOL)), 0xb2);
    }

    #[test]
    fn fn_00c25ca0_constructs_strip_data_or_answers_null() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        e.register(PSYS_DATA_CONSTRUCTOR, |_, _| panic!("no object"));
        assert_eq!(e.call(0x00c2_5ca0, &args![]).u32(), 0);
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(PSYS_DATA_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0] + 0x7c, 0x5555);
            Ret::default()
        });
        let made = e.call(0x00c2_5ca0, &args![]).u32();
        assert_eq!(e.mem.u32(made), STRIP_PSYS_DATA_VTABLE);
        assert_eq!(e.mem.u32(made + 0x7c), 0);
    }
}
