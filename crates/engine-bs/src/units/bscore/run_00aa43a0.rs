//! `BSCore/run_00aa43a0` (Xbox PDB source unit), subsystem `BSCore`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `MemoryManager` helpers: walks over every heap (both arrays, through
//! `00aa46a0`) calling one vtable slot, a global counter changed with
//! interlocked operations, and a global flag byte. The `MemoryManager`
//! counts are read at their PC offsets (+0x02 and +0x04, `u16`; see
//! `run_00aa45a0.rs`).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `MemoryManager` singleton.
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `MemoryManager` heap by index over both arrays (`this`, index).
const HEAP_BY_INDEX: u32 = 0x00aa_46a0;
/// Walks the memory pools calling `00aa7300` on each.
const CHECK_ALL_POOLS: u32 = 0x00aa_7370;
/// A global counter (`u32`), changed with `InterlockedIncrement`/`Decrement`.
const COUNTER: u32 = 0x011f_6234;
/// A global flag byte.
const FLAG: u32 = 0x011f_6230;
/// Import slot of `InterlockedIncrement`.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// Import slot of `InterlockedDecrement`.
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

/// Sum of the `MemoryManager`'s two heap counts (+0x04 and +0x02, `u16`).
fn heap_total(e: &Engine, manager: u32) -> i32 {
    let second = e.mem.u16(manager.wrapping_add(0x04)) as i32;
    let first = e.mem.u16(manager.wrapping_add(0x02)) as i32;
    first + second
}

// Translated from 00aa43a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `00aa7370` (over the memory pools), then calls vtable slot +0x14
/// (Xbox `IMemoryHeap::CheckHeapIntegrity`, unconfirmed on the PC) of every
/// heap of this manager. The stack word is never read. The heap pointer is
/// not checked for null.
pub fn fn_00aa43a0(e: &mut Engine, this: Ptr, _unused_1: u32) {
    e.call(CHECK_ALL_POOLS, &[]);
    let total = heap_total(e, this.addr());
    let mut index: i32 = 0;
    while index < total {
        let heap = e.call(HEAP_BY_INDEX, &args![this, index]).u32();
        e.vcall(heap, 0x14, &[]);
        index += 1;
    }
}

// Translated from 00aa4400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls vtable slot +0x2c of every heap of the `MemoryManager` singleton.
pub fn fn_00aa4400(e: &mut Engine) {
    let total = heap_total(e, MEMORY_MANAGER);
    let mut index: i32 = 0;
    while index < total {
        let heap = e.call(HEAP_BY_INDEX, &args![MEMORY_MANAGER, index]).u32();
        e.vcall(heap, 0x2c, &[]);
        index += 1;
    }
}

// Translated from 00aa4460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterlockedIncrement` of the counter at `011f6234`.
pub fn fn_00aa4460(e: &mut Engine) {
    e.call(INTERLOCKED_INCREMENT, &args![COUNTER]);
}

// Translated from 00aa4470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterlockedDecrement` of the counter at `011f6234`.
pub fn fn_00aa4470(e: &mut Engine) {
    e.call(INTERLOCKED_DECREMENT, &args![COUNTER]);
}

// Translated from 00aa4480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the flag byte at `011f6230` (cdecl, one word).
pub fn fn_00aa4480(e: &mut Engine, value: u8) {
    e.set_global(FLAG, value);
}

// Translated from 00aa4490 (decompiled, FalloutNV.exe 1.4.0.525)
/// True while the counter at `011f6234` is non-zero (unsigned test).
pub fn fn_00aa4490(e: &mut Engine) -> bool {
    e.global::<u32>(COUNTER) > 0
}

// Translated from 00aa44b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The flag byte at `011f6230`.
pub fn fn_00aa44b0(e: &mut Engine) -> u8 {
    e.global(FLAG)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa43a0, fn_00aa43a0(Ptr, u32)),
        entry!(0x00aa4400, fn_00aa4400()),
        entry!(0x00aa4460, fn_00aa4460()),
        entry!(0x00aa4470, fn_00aa4470()),
        entry!(0x00aa4480, fn_00aa4480(u8)),
        entry!(0x00aa4490, fn_00aa4490() -> bool),
        entry!(0x00aa44b0, fn_00aa44b0() -> u8),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEAP_VTABLE: u32 = 0x0300_0000;
    const SLOT_14: u32 = 0x0300_1000;
    const SLOT_2C: u32 = 0x0300_1100;

    /// Doubles: heap `n` is `0x0400_0000 + n * 0x10`, all sharing one
    /// vtable whose slots +0x14 and +0x2c are recorded.
    fn heap_engine() -> Engine {
        let mut e = Engine::new();
        e.register(HEAP_BY_INDEX, |_, a| (0x0400_0000 + a[1] * 0x10).into_ret());
        e.register(CHECK_ALL_POOLS, |_, _| Ret::default());
        e.map(0x0400_0000, 0x1000);
        for n in 0..16 {
            e.mem.set_u32(0x0400_0000 + n * 0x10, HEAP_VTABLE);
        }
        let mut slots = vec![0u32; 12];
        slots[5] = SLOT_14;
        slots[11] = SLOT_2C;
        e.put_vtable(HEAP_VTABLE, &slots);
        e.register(SLOT_14, |_, _| Ret::default());
        e.register(SLOT_2C, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    fn targets(e: &Engine, addr: u32) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w[0])
            .collect()
    }

    #[test]
    fn checks_pools_then_every_heap() {
        let mut e = heap_engine();
        let m: Ptr = Ptr::new(e.mem.alloc(0x10));
        e.mem.set_u16(m.addr() + 2, 2);
        e.mem.set_u16(m.addr() + 4, 1);
        e.call(0x00aa43a0, &args![m, 0u32]);
        let log = e.call_log.as_ref().unwrap();
        let first = |addr: u32| log.iter().position(|(a, _)| *a == addr).unwrap();
        assert!(first(CHECK_ALL_POOLS) < first(HEAP_BY_INDEX));
        assert_eq!(
            targets(&e, HEAP_BY_INDEX),
            vec![m.addr(), m.addr(), m.addr()]
        );
        assert_eq!(
            targets(&e, SLOT_14),
            vec![0x0400_0000, 0x0400_0010, 0x0400_0020]
        );
    }

    #[test]
    fn calls_slot_2c_of_every_singleton_heap() {
        let mut e = heap_engine();
        e.map(0x011f_6000, 0x1000);
        e.set_global(MEMORY_MANAGER + 2, 1u16);
        e.set_global(MEMORY_MANAGER + 4, 1u16);
        e.call(0x00aa4400, &[]);
        assert_eq!(targets(&e, HEAP_BY_INDEX), vec![MEMORY_MANAGER; 2]);
        assert_eq!(targets(&e, SLOT_2C), vec![0x0400_0000, 0x0400_0010]);
        // No heaps: no calls.
        e.set_global(MEMORY_MANAGER + 2, 0u16);
        e.set_global(MEMORY_MANAGER + 4, 0u16);
        e.call_log = Some(vec![]);
        e.call(0x00aa4400, &[]);
        assert!(targets(&e, HEAP_BY_INDEX).is_empty());
    }

    #[test]
    fn counter_changes_through_interlocked_calls() {
        let mut e = Engine::new();
        e.register(INTERLOCKED_INCREMENT, |_, _| Ret::default());
        e.register(INTERLOCKED_DECREMENT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00aa4460, &[]);
        e.call(0x00aa4470, &[]);
        let log = e.call_log.take().unwrap();
        let inc: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == INTERLOCKED_INCREMENT)
            .collect();
        let dec: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == INTERLOCKED_DECREMENT)
            .collect();
        assert_eq!(inc[0].1, vec![COUNTER]);
        assert_eq!(dec[0].1, vec![COUNTER]);
    }

    #[test]
    fn counter_test_and_flag() {
        let mut e = Engine::new();
        e.map(0x011f_6000, 0x1000);
        assert!(!e.call(0x00aa4490, &[]).bool());
        e.set_global(COUNTER, 0x8000_0000u32);
        assert!(e.call(0x00aa4490, &[]).bool());
        e.call(0x00aa4480, &args![0x123u32]);
        assert_eq!(e.global::<u8>(FLAG), 0x23);
        assert_eq!(e.call(0x00aa44b0, &[]).u8(), 0x23);
    }
}
