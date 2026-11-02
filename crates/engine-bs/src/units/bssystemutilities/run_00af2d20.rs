//! `BSSystemUtilities/run_00af2d20` (the engine map's run of unnamed functions from
//! `00af2d20`), subsystem `BSSystemUtilities`: its functions in FalloutNV.exe 1.4.0.525,
//! translated (docs/ENGINE_CRATE.md). `BSSystemUtility::Initialize` and `Destroy` call them
//! as the network utility's start and stop.

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00af2d20 (disassembled, FalloutNV.exe 1.4.0.525)
/// No label (no arguments, plain `RET`): answers 1 (`AL`).
pub fn fn_00af2d20(_e: &mut Engine) -> u8 {
    1
}

// Translated from 00af2d30 (disassembled, FalloutNV.exe 1.4.0.525)
/// No label (no arguments, plain `RET`): does nothing.
pub fn fn_00af2d30(_e: &mut Engine) {}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00af_2d20, fn_00af2d20() -> u8),
        entry!(0x00af_2d30, fn_00af2d30()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_start_answers_one_and_the_stop_does_nothing() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00af_2d20, &args![]).u8(), 1);
        e.call(0x00af_2d30, &args![]);
    }
}
