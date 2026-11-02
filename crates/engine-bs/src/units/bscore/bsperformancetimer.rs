//! `BSCore/bsperformancetimer.cpp` (Xbox PDB source unit), subsystem `BSCore`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0045d260 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `BSPerformanceTimerManager::Register` (Xbox PDB; `thiscall`, `RET 0x10`:
/// four words it never reads): an empty body in this build that returns
/// `this`.
pub fn bs_performance_timer_manager_register(
    _e: &mut Engine,
    this: Ptr,
    _unused_0: u32,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
) -> Ptr {
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x0045d260,
        bs_performance_timer_manager_register(Ptr, u32, u32, u32, u32) -> Ptr
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_this() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0045d260, &[0x1234, 1, 2, 3, 4]).u32(), 0x1234);
    }
}
