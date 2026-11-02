//! `(unplaced)/run_00fd9bf0` (no source unit: the compiler's `atexit` destructors), subsystem
//! `compiler: static init/exit`: its functions in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The global `ProcessLists` object (`011e0e80`) that its static initializer `00fa9890`
/// constructs.
const PROCESS_LISTS: u32 = 0x011e_0e80;

// Translated from 00fd9bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global `ProcessLists` (`011e0e80`): runs the destructor
/// `0096d290` on it (`ECX` = the global). cdecl, no parameters, no result.
pub fn fn_00fd9bf0(e: &mut Engine) {
    e.call(0x0096_d290, &args![PROCESS_LISTS]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00fd_9bf0, fn_00fd9bf0())]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn destructor_runs_on_the_global_process_lists() {
        let mut e = Engine::new();
        e.register(0x0096_d290, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00fd_9bf0, &args![]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(0x0096_d290, vec![0x011e_0e80])]);
    }
}
