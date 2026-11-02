//! `BSHavok/run_00c9f320` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::bhkentity::enter_scope;

/// The routine run on the object held at `+0xC` (`00c8f670`, `this` plus one argument).
const RELEASE_HELD: u32 = 0x00c8_f670;

// Translated from 00c9f320 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c9f320 - the FS:[0] exception frame is not modeled.
/// `fn_00c9f320` (unnamed in the Xbox PDB): takes a flag (only its low byte is tested). With
/// the allocation-scope word set to 0x10, and when the flag is not 0: runs `00c8f670(held, 1)`
/// on the object at `+0xC` when there is one, and clears `+0xC`. The scope word is then put
/// back.
pub fn fn_00c9f320(e: &mut Engine, this: u32, flag: u32) {
    let (word, saved) = enter_scope(e);
    if flag as u8 != 0 {
        let held = e.mem.u32(this + 0xc);
        if held != 0 {
            e.call(RELEASE_HELD, &args![held, 1u32]);
        }
        e.mem.set_u32(this + 0xc, 0);
    }
    e.mem.set_u32(word, saved);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c9f320, fn_00c9f320(u32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bshavok::bhkentity::testing::{calls_to, logging_engine};
    use crate::units::bshavok::bhkentity::SCOPE_WORD;

    #[test]
    fn releases_the_held_object_inside_the_scope() {
        let mut e = logging_engine();
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 7);
        e.register_double(RELEASE_HELD, move |e, _| {
            let word = e.tls() + SCOPE_WORD;
            assert_eq!(e.mem.u32(word), 0x10);
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0xc, 0x4000);
        e.call(0x00c9_f320, &args![this, 0x100u32 | 1]);
        assert_eq!(calls_to(&e, RELEASE_HELD), [[0x4000, 1]]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(word), 7);
    }

    #[test]
    fn clears_the_slot_without_a_call_when_nothing_is_held() {
        let mut e = logging_engine();
        let this = e.mem.alloc(0x20);
        e.call(0x00c9_f320, &args![this, 1u32]);
        assert!(calls_to(&e, RELEASE_HELD).is_empty());
        assert_eq!(e.mem.u32(this + 0xc), 0);
    }

    #[test]
    fn a_zero_flag_leaves_everything_alone() {
        let mut e = logging_engine();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0xc, 0x4000);
        e.call(0x00c9_f320, &args![this, 0x100u32]);
        assert!(calls_to(&e, RELEASE_HELD).is_empty());
        assert_eq!(e.mem.u32(this + 0xc), 0x4000);
        let word = e.tls() + SCOPE_WORD;
        assert_eq!(e.mem.u32(word), 0);
    }
}
