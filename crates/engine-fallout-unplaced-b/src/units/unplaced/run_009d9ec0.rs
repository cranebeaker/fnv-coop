//! `(unplaced)/run_009d9ec0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiPointer<..>::operator=` (`006e5cc0`, engine map; `thiscall`, one word).
const POINTER_ASSIGN: u32 = 0x006e_5cc0;

// Translated from 009d9ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word): assigns `value` to the pointer held at
/// +0xB0 (`006e5cc0`) and passes that call's result back unchanged.
pub fn fn_009d9ec0(e: &mut Engine, this: Ptr, value: u32) -> Ret {
    e.call(POINTER_ASSIGN, &args![this.addr() + 0xb0, value])
}

/// The registered form of `009d9ec0`: `this` and the value, the result passed back whole.
fn fn_009d9ec0_entry(e: &mut Engine, words: &[u32]) -> Ret {
    fn_009d9ec0(e, Ptr::new(words[0]), words[1])
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![(0x009d_9ec0, fn_009d9ec0_entry as AbiFn)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assigns_the_held_pointer() {
        let mut e = Engine::new();
        e.register(POINTER_ASSIGN, |_, a| a[0].into_ret());
        let this = e.mem.alloc(0xc0);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x009d_9ec0, &args![this, 0x5000u32]).u32(),
            this + 0xb0
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(POINTER_ASSIGN, vec![this + 0xb0, 0x5000])]);
    }
}
