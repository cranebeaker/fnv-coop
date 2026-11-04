//! `(unplaced)/run_007d6e90` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable slot called (+0x14).
const SLOT_14: u32 = 0x14;

// Translated from 007d6e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `this`'s virtual slot +0x14 with `(0, 0)` (unnamed in the engine
/// map) and answers its value (the exe ends on that call and leaves its
/// `EAX`, which the callers use as an object).
pub fn fn_007d6e90(e: &mut Engine, this: Ptr) -> u32 {
    e.vcall(this.addr(), SLOT_14, &args![0u32, 0u32]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x007d_6e90, fn_007d6e90(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calls_slot_0x14_with_two_zeros() {
        let mut e = Engine::new();
        e.put_vtable(0x0360_0000, &[0, 0, 0, 0, 0, 0x0350_0000]);
        e.register(0x0350_0000, |_, _| Ret::default());
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, 0x0360_0000);
        e.call_log = Some(vec![]);
        e.call(0x007d_6e90, &args![this]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == 0x0350_0000).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![this, 0, 0]);
    }

    #[test]
    fn answers_slot_0x14s_value() {
        let mut e = Engine::new();
        e.put_vtable(0x0360_0000, &[0, 0, 0, 0, 0, 0x0350_0000]);
        e.register(0x0350_0000, |_, _| 0x4444u32.into_ret());
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, 0x0360_0000);
        assert_eq!(e.call(0x007d_6e90, &args![this]).u32(), 0x4444);
    }
}
