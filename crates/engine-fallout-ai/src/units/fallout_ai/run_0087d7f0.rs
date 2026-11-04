//! `fallout/ai/run_0087d7f0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0087d7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: the word at `this + 0x1a8`.
pub fn fn_0087d7f0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x1a8)
}

// Translated from 0087d810 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: stores `value` at `this + 0x1a8`.
pub fn fn_0087d810(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x1a8, value);
}

/// `Actor::~Actor` (engine map, `0087dcb0`; `thiscall`, no stack words).
const ACTOR_DESTRUCTOR: u32 = 0x0087_dcb0;
/// `00401030` (`cdecl`, one word): frees an object.
const FREE_OBJECT: u32 = 0x0040_1030;

// Translated from 0087d830 (decompiled, FalloutNV.exe 1.4.0.525)
/// A scalar deleting destructor (no map name; `thiscall`, one stack word: the flags, `RET 4`):
/// `Actor::~Actor` on `this`, then `this` freed (`00401030`) when bit 0 of `flags` is set.
/// Returns `this`.
pub fn fn_0087d830(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(ACTOR_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE_OBJECT, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0087_d7f0, fn_0087d7f0(Ptr) -> u32),
        entry!(0x0087_d810, fn_0087d810(Ptr, u32)),
        entry!(0x0087_d830, fn_0087d830(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_at_1a8_round_trips() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x1b0);
        e.call(0x0087_d810, &args![this, 0xdead_beefu32]);
        assert_eq!(e.mem.u32(this + 0x1a8), 0xdead_beef);
        assert_eq!(e.call(0x0087_d7f0, &args![this]).u32(), 0xdead_beef);
    }
}

#[cfg(test)]
mod n027_tests {
    use super::*;

    #[test]
    fn fn_0087d830_destroys_and_frees_on_bit_0() {
        for (flags, frees) in [(1u32, 1usize), (3, 1), (0, 0), (2, 0)] {
            let mut e = Engine::new();
            e.register(ACTOR_DESTRUCTOR, |_, _| Ret::default());
            e.register(FREE_OBJECT, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0087_d830, &args![0x1000u32, flags]).u32(), 0x1000);
            let log = e.call_log.take().unwrap();
            let destroys: Vec<_> = log.iter().filter(|c| c.0 == ACTOR_DESTRUCTOR).collect();
            assert_eq!(destroys.len(), 1);
            assert_eq!(destroys[0].1, vec![0x1000]);
            let freed: Vec<_> = log.iter().filter(|c| c.0 == FREE_OBJECT).collect();
            assert_eq!(freed.len(), frees);
            if frees == 1 {
                assert_eq!(freed[0].1, vec![0x1000]);
            }
        }
    }
}
