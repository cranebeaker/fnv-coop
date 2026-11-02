//! `fallout shared/pathfinding/run_006bc870` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006bc870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): copies the word at `+0`, the half-word at `+4` and the
/// word at `+8` of `other` into `this` and returns `this`.
pub fn fn_006bc870(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    let first = e.mem.u32(other.addr());
    e.mem.set_u32(this.addr(), first);
    let second = e.mem.u16(other.addr().wrapping_add(4));
    e.mem.set_u16(this.addr().wrapping_add(4), second);
    let third = e.mem.u32(other.addr().wrapping_add(8));
    e.mem.set_u32(this.addr().wrapping_add(8), third);
    this
}

/// `006240d0` (`thiscall`): run on both halves by [`fn_006bc8b0`].
const HALF_CONSTRUCT_006240D0: u32 = 0x0062_40d0;

// Translated from 006bc8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `006240d0` on `this` and on `this + 0xc` (`thiscall`, plain
/// `RET`); returns `this`. (Ghidra's library match names it a
/// `SafeSQueue` constructor; the engine map gives it no name.)
pub fn fn_006bc8b0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(HALF_CONSTRUCT_006240D0, &args![this]);
    e.call(HALF_CONSTRUCT_006240D0, &args![this.addr() + 0xc]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006b_c870, fn_006bc870(Ptr, Ptr) -> Ptr),
        entry!(0x006b_c8b0, fn_006bc8b0(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod b035_tests {
    use super::*;

    #[test]
    fn fn_006bc870_copies_two_words_and_a_half_word() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc);
        let other = e.mem.alloc(0xc);
        e.mem.write(this, &[0xee; 0xc]);
        for (offset, value) in [(0u32, 0x1111_2222u32), (4, 0x3333_4444), (8, 0x5555_6666)] {
            e.mem.set_u32(other + offset, value);
        }
        assert_eq!(e.call(0x006b_c870, &args![this, other]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x1111_2222);
        assert_eq!(e.mem.u32(this + 4), 0xeeee_4444);
        assert_eq!(e.mem.u32(this + 8), 0x5555_6666);
    }

    #[test]
    fn fn_006bc8b0_constructs_both_halves() {
        let mut e = Engine::new();
        e.register(HALF_CONSTRUCT_006240D0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006b_c8b0, &args![0x4000u32]).u32(), 0x4000);
        let log = e.call_log.take().unwrap();
        let halves: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == HALF_CONSTRUCT_006240D0)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(halves, vec![vec![0x4000], vec![0x400c]]);
    }
}
