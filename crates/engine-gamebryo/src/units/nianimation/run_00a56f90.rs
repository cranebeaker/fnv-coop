//! `NiAnimation/run_00a56f90` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The critical section the pool is guarded by (`011f4180`) and the KERNEL32.DLL import
/// slots of `EnterCriticalSection` (`00fdf05c`) and `LeaveCriticalSection` (`00fdf100`).
const POOL_LOCK: u32 = 0x011f_4180;
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// The pool (`011a940c`: the array of free entries, `011a9414`: how many are free,
/// `011a941c`: the size the next refill adds) and its refill (`00a56e60`, unit
/// `nigeommorpherupdatetask.cpp`; `thiscall` on the pool, that size).
const POOL: u32 = 0x011a_940c;
const POOL_FREE: u32 = 0x011a_9414;
const POOL_GROWTH: u32 = 0x011a_941c;
const POOL_REFILL: u32 = 0x00a5_6e60;

// Translated from 00a56f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments; the pool the geometry morpher's update tasks
/// come from, by the unit of its refill): under the pool's critical section, an empty pool
/// is refilled first (`00a56e60` with the growth size, which then doubles); then the first
/// free entry is taken, the free count goes down and the last free entry moves into the
/// first slot. Answers the entry.
pub fn fn_00a56f90(e: &mut Engine) -> u32 {
    e.call(ENTER_CRITICAL_SECTION, &args![POOL_LOCK]);
    if e.mem.u32(POOL_FREE) == 0 {
        let growth = e.mem.u32(POOL_GROWTH);
        e.call(POOL_REFILL, &args![POOL, growth]);
        let growth = e.mem.u32(POOL_GROWTH);
        e.mem.set_u32(POOL_GROWTH, growth.wrapping_add(growth));
    }
    let free = e.mem.u32(POOL_FREE);
    let entries = e.mem.u32(POOL);
    let entry = e.mem.u32(entries);
    let free = free.wrapping_sub(1);
    e.mem.set_u32(POOL_FREE, free);
    let last = e.mem.u32(entries.wrapping_add(free.wrapping_mul(4)));
    e.mem.set_u32(entries, last);
    e.call(LEAVE_CRITICAL_SECTION, &args![POOL_LOCK]);
    entry
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a5_6f90, fn_00a56f90() -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn takes_the_first_free_entry_refilling_when_empty() {
        let mut e = Engine::new();
        e.map(0x011a_9000, 0x1000);
        e.map(0x011f_4000, 0x1000);
        e.register(ENTER_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(LEAVE_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(POOL_REFILL, |e, a| {
            let entries = e.mem.alloc(4 * a[1]);
            for i in 0..a[1] {
                e.mem.set_u32(entries + 4 * i, 0x100 + i);
            }
            e.mem.set_u32(POOL, entries);
            e.mem.set_u32(POOL_FREE, a[1]);
            Ret::default()
        });
        e.mem.set_u32(POOL_GROWTH, 3);
        assert_eq!(e.call(0x00a5_6f90, &[]).u32(), 0x100);
        assert_eq!(e.mem.u32(POOL_GROWTH), 6);
        assert_eq!(e.mem.u32(POOL_FREE), 2);
        // The last free entry moved to the front.
        assert_eq!(e.call(0x00a5_6f90, &[]).u32(), 0x102);
        assert_eq!(e.call(0x00a5_6f90, &[]).u32(), 0x101);
        assert_eq!(e.mem.u32(POOL_FREE), 0);
    }
}
