//! `BSCore/run_00aa6590` (Xbox PDB source unit), subsystem `BSCore`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::bscore::memorypool::MemoryPool;

/// Import slot of `VirtualAlloc`, and its flags.
const VIRTUAL_ALLOC: u32 = 0x00fd_f114;
const MEM_RESERVE: u32 = 0x2000;
const PAGE_READWRITE: u32 = 4;

// Translated from 00aa65b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reserves the pool's range (`thiscall`): until `pHeap` is set, tries
/// `VirtualAlloc(n << 24, iPoolSize, MEM_RESERVE, PAGE_READWRITE)` at the
/// 16 MiB boundaries n = 1, 2, ... 0xff, and stops after the last.
pub fn fn_00aa65b0(e: &mut Engine, this: Ptr<MemoryPool>) {
    let mut slot: u32 = 1;
    while e.get(this, MemoryPool::pHeap) == 0 {
        let size = e.get(this, MemoryPool::iPoolSize);
        let heap = e
            .call(
                VIRTUAL_ALLOC,
                &args![slot << 24, size, MEM_RESERVE, PAGE_READWRITE],
            )
            .u32();
        e.set(this, MemoryPool::pHeap, heap);
        if slot >= 0xff {
            break;
        }
        slot += 1;
    }
}

/// Import slot of KERNEL32 `ExitThread` (stdcall, one word; it does not return).
const EXIT_THREAD: u32 = 0x00fd_f148;

// Translated from 00aa6590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a thread's stop step (`thiscall`, two stack words, `RET 8`;
/// `this` and the second word are not read): `ExitThread(exit_code)`. The exe's code after
/// the call only returns.
pub fn fn_00aa6590(e: &mut Engine, _this: Ptr, exit_code: u32, _unused_2: u32) {
    e.call(EXIT_THREAD, &args![exit_code]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa_6590, fn_00aa6590(Ptr, u32, u32)),
        entry!(0x00aa65b0, fn_00aa65b0(Ptr<MemoryPool>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tries(log: &[(u32, Vec<u32>)]) -> Vec<u32> {
        log.iter()
            .filter(|(a, _)| *a == VIRTUAL_ALLOC)
            .map(|(_, w)| w[0])
            .collect()
    }

    #[test]
    fn tries_16_mib_boundaries_until_one_is_free() {
        let mut e = Engine::new();
        // Free from 0x0300_0000 on.
        e.register(VIRTUAL_ALLOC, |_, a| {
            (if a[0] >= 0x0300_0000 { a[0] } else { 0 }).into_ret()
        });
        let pool: Ptr<MemoryPool> = e.new_object();
        e.set(pool, MemoryPool::iPoolSize, 0x10_0000);
        e.call_log = Some(vec![]);
        e.call(0x00aa_65b0, &args![pool]);
        assert_eq!(e.get(pool, MemoryPool::pHeap), 0x0300_0000);
        let log = e.call_log.take().unwrap();
        assert_eq!(tries(&log), vec![0x0100_0000, 0x0200_0000, 0x0300_0000]);
        let first = log.iter().find(|(a, _)| *a == VIRTUAL_ALLOC).unwrap();
        assert_eq!(first.1[1..], [0x10_0000, MEM_RESERVE, PAGE_READWRITE]);
    }

    #[test]
    fn gives_up_after_slot_0xff_and_skips_a_set_heap() {
        let mut e = Engine::new();
        e.register(VIRTUAL_ALLOC, |_, _| 0u32.into_ret());
        let pool: Ptr<MemoryPool> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x00aa_65b0, &args![pool]);
        let log = e.call_log.take().unwrap();
        let tried = tries(&log);
        assert_eq!(tried.len(), 0xff);
        assert_eq!(*tried.last().unwrap(), 0xff00_0000);
        // Already reserved: nothing is tried.
        e.set(pool, MemoryPool::pHeap, 0x0100_0000);
        e.call_log = Some(vec![]);
        e.call(0x00aa_65b0, &args![pool]);
        assert!(tries(&e.call_log.take().unwrap()).is_empty());
    }
}

#[cfg(test)]
mod n082_tests {
    use super::*;

    #[test]
    fn fn_00aa6590_exits_the_thread_with_the_code() {
        let mut e = Engine::new();
        e.register(EXIT_THREAD, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00aa_6590, &args![0x5000u32, 3u32, 9u32]);
        let exits: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == EXIT_THREAD)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(exits, vec![vec![3]]);
    }
}
