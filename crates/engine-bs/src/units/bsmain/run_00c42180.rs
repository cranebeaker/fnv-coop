//! `BSMain/run_00c42180` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A per-thread slot table of the lock-free task containers (constructed by
//! `00c3ec80`, `00c3eeb0`, `00c40b70` and `00c41330`): a count, a Win32 TLS
//! index and an array of `count` two-word entries whose second word owns an
//! object. No Xbox PDB names at these addresses.

#[allow(unused_imports)]
use crate::prelude::*;

/// The `MemoryManager` singleton (`this` of its allocate and free).
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `MemoryManager::Allocate` (thiscall(size)).
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// `MemoryManager::Deallocate` (thiscall(block)).
const MEMORY_FREE: u32 = 0x00aa_4060;
/// Import slot of `TlsAlloc` (stdcall, no words).
const TLS_ALLOC: u32 = 0x00fd_f048;
/// Import slot of `TlsFree` (stdcall(index)).
const TLS_FREE: u32 = 0x00fd_f050;
/// The entry object's drain (`00c42b60`, thiscall; `BSMain/run_00c42960`).
const ENTRY_OBJECT_DRAIN: u32 = 0x00c4_2b60;
/// The thread's TLS word that selects the allocator context; the
/// constructor sets it to 6 around its allocation and restores it.
const TLS_ALLOCATION_CONTEXT: u32 = 0x2b4;
/// The context it sets.
const TASK_CONTEXT: u32 = 6;

layout! {
    /// The slot table (16 bytes; offsets from the PC code).
    pub struct TaskSlotTable: 0x10 {
        /// The number of entries.
        0x00 count: u32,
        /// The Win32 TLS index from `TlsAlloc`.
        0x04 tls_index: u32,
        /// The entry array (`count` entries of two words), null when its
        /// allocation failed.
        0x08 entries: u32,
        /// Cleared by the constructor.
        0x0C unused_0c: u32,
    }
}

// Translated from 00c42180 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c42180 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// The table's constructor: with the thread's allocation context set to 6
/// (and restored at the end), clears +0x0C, stores `count`, allocates
/// `count * 8` bytes (all ones when the product overflows) and clears both
/// words of each entry (the code counts down from `count` with a signed
/// test on `count - 1`, so a count from `0x80000001` up clears nothing), stores the array
/// (null when the allocation failed) and the index `TlsAlloc` gives.
/// Returns `this`.
pub fn fn_00c42180(e: &mut Engine, this: Ptr<TaskSlotTable>, count: u32) -> Ptr<TaskSlotTable> {
    let context = e.tls() + TLS_ALLOCATION_CONTEXT;
    let previous = e.mem.u32(context);
    e.mem.set_u32(context, TASK_CONTEXT);
    e.set(this, TaskSlotTable::unused_0c, 0);
    e.set(this, TaskSlotTable::count, count);
    let count = e.get(this, TaskSlotTable::count);
    let bytes = count as u64 * 8;
    let size = (bytes as u32) | if bytes > u32::MAX as u64 { u32::MAX } else { 0 };
    let block = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, size]).u32();
    let entries = if block == 0 {
        0
    } else {
        let mut remaining = count as i32;
        let mut entry = block;
        loop {
            remaining = remaining.wrapping_sub(1);
            if remaining < 0 {
                break;
            }
            e.mem.set_u32(entry, 0);
            e.mem.set_u32(entry.wrapping_add(4), 0);
            entry = entry.wrapping_add(8);
        }
        block
    };
    e.set(this, TaskSlotTable::entries, entries);
    let index = e.call(TLS_ALLOC, &args![]).u32();
    e.set(this, TaskSlotTable::tls_index, index);
    let context = e.tls() + TLS_ALLOCATION_CONTEXT;
    e.mem.set_u32(context, previous);
    this
}

// Translated from 00c422d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The table's destructor body: for each entry (the count re-read each
/// time) whose second word holds an object, drains it (`00c42b60`) and
/// frees it; then frees the array and the TLS index (`TlsFree`).
pub fn fn_00c422d0(e: &mut Engine, this: Ptr<TaskSlotTable>) {
    let mut index = 0u32;
    while index < e.get(this, TaskSlotTable::count) {
        let entries = e.get(this, TaskSlotTable::entries);
        let object = e
            .mem
            .u32(entries.wrapping_add(index.wrapping_mul(8)).wrapping_add(4));
        if object != 0 {
            e.call(ENTRY_OBJECT_DRAIN, &args![object]);
            // `MOV ECX,1; AND ECX,1; JZ`: the free is never skipped.
            e.call(MEMORY_FREE, &args![MEMORY_MANAGER, object]);
        }
        index = index.wrapping_add(1);
    }
    let entries = e.get(this, TaskSlotTable::entries);
    e.call(MEMORY_FREE, &args![MEMORY_MANAGER, entries]);
    let tls_index = e.get(this, TaskSlotTable::tls_index);
    e.call(TLS_FREE, &args![tls_index]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c4_2180,
            fn_00c42180(Ptr<TaskSlotTable>, u32) -> Ptr<TaskSlotTable>
        ),
        entry!(0x00c4_22d0, fn_00c422d0(Ptr<TaskSlotTable>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type CallLog = [(u32, Vec<u32>)];

    fn calls(log: &CallLog, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(MEMORY_FREE, |_, _| Ret::default());
        e.register(TLS_ALLOC, |_, _| 7u32.into_ret());
        e.register(TLS_FREE, |_, _| Ret::default());
        e.register(ENTRY_OBJECT_DRAIN, |_, _| Ret::default());
        e
    }

    #[test]
    fn constructor_allocates_cleared_entries_and_a_tls_index() {
        let mut e = engine();
        let context = e.tls() + TLS_ALLOCATION_CONTEXT;
        e.mem.set_u32(context, 3);
        e.register(MEMORY_ALLOCATE, |e, a| {
            let block = e.mem.alloc(a[1]);
            for offset in (0..a[1]).step_by(4) {
                e.mem.set_u32(block + offset, 0xdead_beef);
            }
            // The context the allocation sees.
            let context = e.tls() + TLS_ALLOCATION_CONTEXT;
            assert_eq!(e.mem.u32(context), TASK_CONTEXT);
            block.into_ret()
        });
        let this: Ptr<TaskSlotTable> = e.new_object();
        e.set(this, TaskSlotTable::unused_0c, 9);
        e.call_log = Some(vec![]);
        let back = e.call(0x00c4_2180, &args![this, 3u32]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(back, this.addr());
        assert_eq!(calls(&log, MEMORY_ALLOCATE), vec![vec![MEMORY_MANAGER, 24]]);
        let entries = e.get(this, TaskSlotTable::entries);
        for offset in (0..24).step_by(4) {
            assert_eq!(e.mem.u32(entries + offset), 0);
        }
        assert_eq!(e.get(this, TaskSlotTable::count), 3);
        assert_eq!(e.get(this, TaskSlotTable::tls_index), 7);
        assert_eq!(e.get(this, TaskSlotTable::unused_0c), 0);
        assert_eq!(e.mem.u32(context), 3);
    }

    #[test]
    fn constructor_with_a_failed_allocation_or_an_overflowing_size() {
        let mut e = engine();
        e.register(MEMORY_ALLOCATE, |_, _| Ret::default());
        let this: Ptr<TaskSlotTable> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x00c4_2180, &args![this, 0x2000_0000u32]);
        let log = e.call_log.take().unwrap();
        // 0x2000_0000 * 8 overflows: the size is all ones.
        assert_eq!(
            calls(&log, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, u32::MAX]]
        );
        assert_eq!(e.get(this, TaskSlotTable::entries), 0);
        assert_eq!(e.get(this, TaskSlotTable::tls_index), 7);
    }

    #[test]
    fn destructor_drains_and_frees_each_object() {
        let mut e = engine();
        let this: Ptr<TaskSlotTable> = e.new_object();
        let entries = e.mem.alloc(24);
        e.mem.set_u32(entries + 4, 0x5000);
        e.mem.set_u32(entries + 20, 0x6000);
        e.set(this, TaskSlotTable::count, 3);
        e.set(this, TaskSlotTable::entries, entries);
        e.set(this, TaskSlotTable::tls_index, 4);
        e.call_log = Some(vec![]);
        e.call(0x00c4_22d0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, ENTRY_OBJECT_DRAIN),
            vec![vec![0x5000], vec![0x6000]]
        );
        assert_eq!(
            calls(&log, MEMORY_FREE),
            vec![
                vec![MEMORY_MANAGER, 0x5000],
                vec![MEMORY_MANAGER, 0x6000],
                vec![MEMORY_MANAGER, entries]
            ]
        );
        assert_eq!(calls(&log, TLS_FREE), vec![vec![4]]);
    }
}
