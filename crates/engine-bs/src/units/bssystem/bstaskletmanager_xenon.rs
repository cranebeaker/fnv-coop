//! `BSSystem/bstaskletmanager_xenon.cpp` (Xbox PDB source unit), subsystem `BSSystem`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The tasklet manager's function-local static: its guard word (`011f8ae8`, bit 0), the object
/// (`011f8270`), its constructor (`00b01d50`, `thiscall`, one word: the thread count) and its
/// `_atexit` destructor (`00fdc400`).
const GUARD: u32 = 0x011f_8ae8;
const MANAGER: u32 = 0x011f_8270;
const MANAGER_CONSTRUCT: u32 = 0x00b0_1d50;
const MANAGER_DESTRUCTOR: u32 = 0x00fd_c400;
/// `_atexit` (`00ec658f`, `cdecl`, one word).
const ATEXIT: u32 = 0x00ec_658f;

// Translated from 00b00a00 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b00a00 - the FS:[0] exception frame (the unwind of the static's
// construction) is not modeled.
/// `BSTaskletManager::QInstance` (engine map; `cdecl`, no arguments, plain `RET`): the first time
/// (bit 0 of `011f8ae8` clear) sets the bit, constructs the manager at `011f8270` with 6 threads
/// (`00b01d50`) and gives its destructor `00fdc400` to `_atexit`. Answers `011f8270`.
pub fn bs_tasklet_manager_q_instance(e: &mut Engine) -> u32 {
    if e.mem.u32(GUARD) & 1 == 0 {
        let guard = e.mem.u32(GUARD);
        e.mem.set_u32(GUARD, guard | 1);
        e.call(MANAGER_CONSTRUCT, &args![MANAGER, 6u32]);
        e.call(ATEXIT, &args![MANAGER_DESTRUCTOR]);
    }
    MANAGER
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00b0_0a00, bs_tasklet_manager_q_instance() -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_manager_is_made_once_with_six_threads() {
        let mut e = Engine::new();
        e.map(0x011f_8000, 0x1000);
        e.register(MANAGER_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(ATEXIT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b0_0a00, &args![]).u32(), MANAGER);
        assert_eq!(e.call(0x00b0_0a00, &args![]).u32(), MANAGER);
        let log = e.call_log.take().unwrap();
        let made: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == MANAGER_CONSTRUCT)
            .collect();
        assert_eq!(made.len(), 1);
        assert_eq!(made[0].1, [MANAGER, 6]);
        let registered: Vec<_> = log.iter().filter(|(a, _)| *a == ATEXIT).collect();
        assert_eq!(registered[0].1, [MANAGER_DESTRUCTOR]);
    }
}
