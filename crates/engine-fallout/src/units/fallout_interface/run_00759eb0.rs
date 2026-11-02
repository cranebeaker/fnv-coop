//! `fallout/interface/run_00759eb0` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSString`'s constructor (`004037b0`), `006ffe10` (`thiscall`) and
/// `AnimatingText::ResetBuffer` (`00759f20`, `RET 4`).
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const INIT_006FFE10: u32 = 0x006f_fe10;
const ANIMATING_TEXT_RESET_BUFFER: u32 = 0x0075_9f20;

// Translated from 00759eb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00759eb0 - the C++ exception-unwinding frame (FS:[0] around the string) is not translated.
/// Unnamed (`thiscall`, `RET 4`; an `AnimatingText` constructor by its last call): the
/// `BSString` at +0 constructed, `006ffe10`, then `AnimatingText::ResetBuffer(value)`.
/// Answers `this`.
pub fn fn_00759eb0(e: &mut Engine, this: Ptr, value: u32) -> Ptr {
    e.call(STRING_CONSTRUCT, &args![this]);
    e.call(INIT_006FFE10, &args![this]);
    e.call(ANIMATING_TEXT_RESET_BUFFER, &args![this, value]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0075_9eb0, fn_00759eb0(Ptr, u32) -> Ptr)]
}

#[cfg(test)]
mod n059_tests {
    use super::*;

    #[test]
    fn constructs_and_resets_the_buffer() {
        let mut e = Engine::new();
        for address in [STRING_CONSTRUCT, INIT_006FFE10, ANIMATING_TEXT_RESET_BUFFER] {
            e.register(address, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0075_9eb0, &args![0x5000u32, 0x77u32]).u32(),
            0x5000
        );
        let log: Vec<_> = e.call_log.unwrap().into_iter().skip(1).collect();
        assert_eq!(
            log,
            vec![
                (STRING_CONSTRUCT, vec![0x5000]),
                (INIT_006FFE10, vec![0x5000]),
                (ANIMATING_TEXT_RESET_BUFFER, vec![0x5000, 0x77]),
            ]
        );
    }
}
