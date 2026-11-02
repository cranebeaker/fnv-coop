//! `FaceGen/run_00ab9bc0` (Xbox PDB source unit), subsystem `FaceGen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Two members of a vector of 12-byte elements (the library vector of 0x18 bytes: proxy,
//! allocator at +0x08, first, last and end at +0x0C, +0x10, +0x14).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::facegen::binaryfile::{VECTOR_FIRST, VECTOR_LAST};
use crate::units::facegen::run_00aba3c0::{checked_iterator, VECTOR_ALLOCATOR, VECTOR_END};

/// The 12-byte elements' uninitialized fill (`00ac20d0`, `cdecl`, six words: destination,
/// count, the value's address, the allocator and two tags), the vector's insert
/// (`00aba3c0`, `thiscall`: the result iterator, the position's proxy and pointer, the value's
/// address) and its erase (`00aba9a0`, `thiscall`: the result iterator and the first and last
/// iterators' proxies and pointers), unit `run_00aba3c0`.
const FILL_12: u32 = 0x00ac_20d0;
const INSERT_12: u32 = 0x00ab_a3c0;
const ERASE_12: u32 = 0x00ab_a9a0;

// Translated from 00ab9bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the 12-byte element vector's `push_back` (`thiscall`, the value's
/// address). With room left (the size, a signed division by 12, below the capacity, 0 for a
/// null first pointer) the value is constructed at the end (`00ac20d0` with the count 1, the
/// allocator, an uninitialized tag byte and 0; the callee reads neither tag) and the end moves
/// by 12; otherwise it is inserted at the end (`00aba3c0`, the end's iterator checked first).
pub fn fn_00ab9bc0(e: &mut Engine, this: Ptr, value: u32) {
    let vector = this.addr();
    let span = |e: &Engine, field: u32| {
        (e.mem
            .u32(vector + field)
            .wrapping_sub(e.mem.u32(vector + VECTOR_FIRST)) as i32
            / 12) as u32
    };
    let size = span(e, VECTOR_LAST);
    let capacity = if e.mem.u32(vector + VECTOR_FIRST) == 0 {
        0
    } else {
        span(e, VECTOR_END)
    };
    if size < capacity {
        let last = e.mem.u32(vector + VECTOR_LAST);
        e.call(
            FILL_12,
            &args![last, 1u32, value, vector + VECTOR_ALLOCATOR, 0u32, 0u32],
        );
        e.mem.set_u32(vector + VECTOR_LAST, last.wrapping_add(12));
        return;
    }
    let last = e.mem.u32(vector + VECTOR_LAST);
    let proxy = checked_iterator(e, vector, last);
    e.with_stack(8, |e, result| {
        e.call(INSERT_12, &args![this, result, proxy, last, value]);
    });
}

// Translated from 00ab9cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the 12-byte element vector's `clear` (`thiscall`): the iterators
/// at the end and at the start are made (each checked to lie in the vector) and everything
/// between them is erased (`00aba9a0`).
pub fn fn_00ab9cd0(e: &mut Engine, this: Ptr) {
    let vector = this.addr();
    let last = e.mem.u32(vector + VECTOR_LAST);
    let last_proxy = checked_iterator(e, vector, last);
    let first = e.mem.u32(vector + VECTOR_FIRST);
    let first_proxy = checked_iterator(e, vector, first);
    e.with_stack(8, |e, result| {
        e.call(
            ERASE_12,
            &args![this, result, first_proxy, first, last_proxy, last],
        );
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ab_9bc0, fn_00ab9bc0(Ptr, u32)),
        entry!(0x00ab_9cd0, fn_00ab9cd0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::facegen::binaryfile::INVALID_PARAMETER;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn vector(e: &mut Engine, size: u32, capacity: u32) -> u32 {
        let vector = e.mem.alloc(0x18);
        let block = e.mem.alloc(12 * capacity.max(1));
        e.mem.set_u32(vector, 0x77);
        e.mem.set_u32(vector + VECTOR_FIRST, block);
        e.mem.set_u32(vector + VECTOR_LAST, block + 12 * size);
        e.mem.set_u32(vector + VECTOR_END, block + 12 * capacity);
        vector
    }

    #[test]
    fn push_back_fills_in_place_or_inserts() {
        let mut e = Engine::new();
        for addr in [FILL_12, INSERT_12, INVALID_PARAMETER] {
            e.register(addr, |_, _| Ret::default());
        }
        let v = vector(&mut e, 1, 2);
        let block = e.mem.u32(v + VECTOR_FIRST);
        e.call_log = Some(vec![]);
        e.call(0x00ab_9bc0, &args![v, 0x500u32]);
        // Full now: the next one goes through the insert.
        e.call(0x00ab_9bc0, &args![v, 0x600u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, FILL_12),
            vec![vec![block + 12, 1, 0x500, v + 8, 0, 0]]
        );
        assert_eq!(e.mem.u32(v + VECTOR_LAST), block + 24);
        let inserts = calls_to(&log, INSERT_12);
        assert_eq!(inserts.len(), 1);
        assert_eq!(inserts[0][0], v);
        assert_eq!(inserts[0][2..], [0x77, block + 24, 0x600]);
        assert!(calls_to(&log, INVALID_PARAMETER).is_empty());
        // A vector without storage has no capacity.
        let empty = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        e.call(0x00ab_9bc0, &args![empty, 0x700u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, INSERT_12).len(), 1);
        assert!(calls_to(&log, FILL_12).is_empty());
    }

    #[test]
    fn clear_erases_from_first_to_last() {
        let mut e = Engine::new();
        for addr in [ERASE_12, INVALID_PARAMETER] {
            e.register(addr, |_, _| Ret::default());
        }
        let v = vector(&mut e, 2, 3);
        let block = e.mem.u32(v + VECTOR_FIRST);
        e.call_log = Some(vec![]);
        e.call(0x00ab_9cd0, &args![v]);
        let log = e.call_log.take().unwrap();
        let erases = calls_to(&log, ERASE_12);
        assert_eq!(erases.len(), 1);
        assert_eq!(erases[0][0], v);
        assert_eq!(erases[0][2..], [0x77, block, 0x77, block + 24]);
        assert!(calls_to(&log, INVALID_PARAMETER).is_empty());
    }
}
