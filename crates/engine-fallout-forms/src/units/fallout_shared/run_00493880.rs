//! `fallout shared/run_00493880` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00493880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a flags byte; `RET 8`): sets the
/// bits of `mask` in the byte at `this` when `set` is non-zero, clears them
/// otherwise.
pub fn fn_00493880(e: &mut Engine, this: Ptr, set: u8, mask: u8) {
    let flags = e.mem.u8(this.addr());
    let flags = if set != 0 {
        flags | mask
    } else {
        flags & !mask
    };
    e.mem.set_u8(this.addr(), flags);
}

// Translated from 004938c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): the byte at
/// +0x384 of `this` (an object's field, unnamed here).
pub fn fn_004938c0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x384)
}

// Translated from 004938e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): whether any of
/// the low four bits of `008846e0`'s answer for `this` (a `fallout/ai/actor.cpp`
/// function, by the engine map) is set.
pub fn fn_004938e0(e: &mut Engine, this: Ptr) -> bool {
    e.call(0x0088_46e0, &args![this]).u32() & 0xf != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00493880, fn_00493880(Ptr, u8, u8)),
        entry!(0x004938c0, fn_004938c0(Ptr) -> u8),
        entry!(0x004938e0, fn_004938e0(Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flag_bits_are_set_and_cleared() {
        let mut e = Engine::new();
        let flags = e.mem.alloc(4);
        e.mem.set_u8(flags, 0x81);
        e.call(0x0049_3880, &args![flags, 1u32, 0x06u32]);
        assert_eq!(e.mem.u8(flags), 0x87);
        e.call(0x0049_3880, &args![flags, 0u32, 0x03u32]);
        assert_eq!(e.mem.u8(flags), 0x84);
        // Only the low byte of `set` counts.
        e.call(0x0049_3880, &args![flags, 0x100u32, 0x80u32]);
        assert_eq!(e.mem.u8(flags), 0x04);
    }

    #[test]
    fn byte_at_0x384() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x400);
        e.mem.set_u8(object + 0x384, 0x5a);
        assert_eq!(e.call(0x0049_38c0, &args![object]).u8(), 0x5a);
    }

    #[test]
    fn low_four_bits_of_the_answer() {
        let mut e = Engine::new();
        e.register(0x0088_46e0, |_, a| (a[0] & 0xff).into_ret());
        assert!(e.call(0x0049_38e0, &args![0x1008u32]).bool());
        assert!(!e.call(0x0049_38e0, &args![0x10f0u32]).bool());
    }
}
