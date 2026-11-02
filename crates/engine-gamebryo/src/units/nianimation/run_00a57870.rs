//! `NiAnimation/run_00a57870` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `InterlockedIncrement` and `InterlockedDecrement` (import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The pooled empty string's handle (`0109b220`), which is never counted.
const EMPTY_FIXED_STRING: u32 = 0x0109_b220;

// Translated from 00a57870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: releases the `NiFixedString` handle at `this + 4` (the
/// pooled string's count at `handle - 8`, `InterlockedDecrement`) unless it is the empty
/// string's handle (`0109b220`). `thiscall`, plain `RET`.
pub fn fn_00a57870(e: &mut Engine, this: Ptr) {
    let handle = e.mem.u32(this.addr() + 4);
    if handle != e.global::<u32>(EMPTY_FIXED_STRING) {
        e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
    }
}

// Translated from 00a57890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: assigns the `NiFixedString` handle at `source` (an address)
/// to the one at `this + 4` (`thiscall`, `RET 4`). When the handles differ the new one is
/// counted, the old one released (neither when it is the empty string `0109b220`), then
/// stored.
pub fn fn_00a57890(e: &mut Engine, this: Ptr, source: Ptr) {
    let handle = e.mem.u32(source.addr());
    let slot = this.addr() + 4;
    if e.mem.u32(slot) == handle {
        return;
    }
    if handle != e.global::<u32>(EMPTY_FIXED_STRING) {
        e.call(INTERLOCKED_INCREMENT, &args![handle.wrapping_sub(8)]);
    }
    let old = e.mem.u32(slot);
    if old != e.global::<u32>(EMPTY_FIXED_STRING) {
        e.call(INTERLOCKED_DECREMENT, &args![old.wrapping_sub(8)]);
    }
    e.mem.set_u32(slot, handle);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a5_7870, fn_00a57870(Ptr)),
        entry!(0x00a5_7890, fn_00a57890(Ptr, Ptr)),
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
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(EMPTY_FIXED_STRING, 4);
        e.set_global(EMPTY_FIXED_STRING, 0x0900u32);
        e.register(INTERLOCKED_INCREMENT, |_, _| 1u32.into_ret());
        e.register(INTERLOCKED_DECREMENT, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn release_skips_the_empty_string() {
        let mut e = engine();
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0x2008);
        e.call(0x00a5_7870, &args![this]);
        e.mem.set_u32(this + 4, 0x0900);
        e.call(0x00a5_7870, &args![this]);
        assert_eq!(calls(&e, INTERLOCKED_DECREMENT), vec![vec![0x2000]]);
    }

    #[test]
    fn assignment_counts_the_new_handle_and_releases_the_old_one() {
        let mut e = engine();
        let this = e.mem.alloc(8);
        let source = e.mem.alloc(4);
        e.mem.set_u32(this + 4, 0x2008);
        e.mem.set_u32(source, 0x3008);
        e.call(0x00a5_7890, &args![this, source]);
        assert_eq!(e.mem.u32(this + 4), 0x3008);
        assert_eq!(calls(&e, INTERLOCKED_INCREMENT), vec![vec![0x3000]]);
        assert_eq!(calls(&e, INTERLOCKED_DECREMENT), vec![vec![0x2000]]);
        // The same handle again: nothing happens.
        e.call(0x00a5_7890, &args![this, source]);
        assert_eq!(calls(&e, INTERLOCKED_INCREMENT).len(), 1);
        // To and from the empty string: neither side is counted.
        e.mem.set_u32(source, 0x0900);
        e.call(0x00a5_7890, &args![this, source]);
        assert_eq!(calls(&e, INTERLOCKED_INCREMENT).len(), 1);
        assert_eq!(calls(&e, INTERLOCKED_DECREMENT).len(), 2);
        e.mem.set_u32(source, 0x4008);
        e.call(0x00a5_7890, &args![this, source]);
        assert_eq!(calls(&e, INTERLOCKED_INCREMENT).len(), 2);
        assert_eq!(calls(&e, INTERLOCKED_DECREMENT).len(), 2);
        assert_eq!(e.mem.u32(this + 4), 0x4008);
    }
}
