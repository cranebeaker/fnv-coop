//! `(unplaced)/run_00621480` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `HighProcess::GetCurrentProcessIdle` (Xbox PDB, `006214b0`, `thiscall`): the dword at
/// `this + 0x350`.
const HIGH_PROCESS_GET_CURRENT_PROCESS_IDLE: u32 = 0x0062_14b0;

// Translated from 00621480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `cdecl`, one stack word (a process pointer; the saved `this`
/// register is not used): zero for null, otherwise `HighProcess::GetCurrentProcessIdle` of it.
pub fn fn_00621480(e: &mut Engine, process: Ptr) -> u32 {
    if process.is_null() {
        0
    } else {
        e.call(HIGH_PROCESS_GET_CURRENT_PROCESS_IDLE, &args![process])
            .u32()
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0062_1480, fn_00621480(Ptr) -> u32)]
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    #[test]
    fn at_00621480_forwards_a_process_and_answers_zero_for_null() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(HIGH_PROCESS_GET_CURRENT_PROCESS_IDLE, |_, a| {
            (a[0] + 1).into_ret()
        });
        assert_eq!(e.call(0x0062_1480, &args![0u32]).u32(), 0);
        assert!(e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .all(|c| c.0 != HIGH_PROCESS_GET_CURRENT_PROCESS_IDLE));
        assert_eq!(e.call(0x0062_1480, &args![0x4000u32]).u32(), 0x4001);
    }
}
