//! `NiMain/run_00a7e930` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a59b50` (no stack words, unit `run_00a59b50`): builds the object `00a7e930` keeps.
const BUILD_OBJECT: u32 = 0x00a5_9b50;
/// `011f4aa8`: where `00a7e930` keeps the result.
const BUILT_OBJECT_SLOT: u32 = 0x011f_4aa8;

// Translated from 00a7e930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (range inside the NiMain subsystem; run by the start-up
/// `00a89cc0`, no arguments): stores what `00a59b50` answers in `011f4aa8`.
pub fn fn_00a7e930(e: &mut Engine) {
    let result = e.call(BUILD_OBJECT, &args![]).u32();
    e.mem.set_u32(BUILT_OBJECT_SLOT, result);
}

/// `00aa10f0` (`cdecl`, one word): frees a block.
const FREE_BLOCK: u32 = 0x00aa_10f0;

// Translated from 00a7e940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments; one of the `NiMain` shutdown steps of
/// `00a8a280`): frees what `00a7e930` stored in `011f4aa8` (`00aa10f0`, also for null) and
/// clears the pointer.
pub fn fn_00a7e940(e: &mut Engine) {
    let block = e.mem.u32(BUILT_OBJECT_SLOT);
    e.call(FREE_BLOCK, &args![block]);
    e.mem.set_u32(BUILT_OBJECT_SLOT, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a7_e930, fn_00a7e930()),
        entry!(0x00a7_e940, fn_00a7e940()),
    ]
}

#[cfg(test)]
mod start_up_tests {
    use super::*;

    #[test]
    fn stores_the_answer_of_the_callee() {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        e.register(BUILD_OBJECT, |_, _| 0x0abc_def0u32.into_ret());
        e.call(0x00a7_e930, &args![]);
        assert_eq!(e.mem.u32(BUILT_OBJECT_SLOT), 0x0abc_def0);
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    #[test]
    fn fn_00a7e940_frees_the_block_and_clears_the_pointer() {
        let mut e = Engine::new();
        e.map(BUILT_OBJECT_SLOT & !0xfff, 0x1000);
        e.register(FREE_BLOCK, |_, _| Ret::default());
        e.mem.set_u32(BUILT_OBJECT_SLOT, 0x7000);
        e.call_log = Some(vec![]);
        e.call(0x00a7_e940, &args![]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(FREE_BLOCK, vec![0x7000])]);
        assert_eq!(e.mem.u32(BUILT_OBJECT_SLOT), 0);
    }
}
