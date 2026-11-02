//! `Havok SDK/hkwin32criticalsection.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The import slot of `InitializeCriticalSectionAndSpinCount` (`kernel32`,
/// `stdcall`: critical section, spin count).
const INITIALIZE_CRITICAL_SECTION_AND_SPIN_COUNT: u32 = 0x00fd_f130;

// Translated from 00cf6da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCriticalSection::hkCriticalSection` (Xbox PDB), `thiscall`, one stack word:
/// initialises the section at `this` with `spin_count` through
/// `InitializeCriticalSectionAndSpinCount` (the import's result is ignored).
/// Returns `this`.
pub fn hk_critical_section_hk_critical_section(e: &mut Engine, this: Ptr, spin_count: u32) -> Ptr {
    e.call(
        INITIALIZE_CRITICAL_SECTION_AND_SPIN_COUNT,
        &args![this, spin_count],
    );
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00cf6da0,
        hk_critical_section_hk_critical_section(Ptr, u32) -> Ptr
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_constructor_initialises_the_section_with_the_spin_count() {
        let mut e = Engine::new();
        e.register(INITIALIZE_CRITICAL_SECTION_AND_SPIN_COUNT, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(0x00cf_6da0, &args![0x5000u32, 0xfa0u32]);
        assert_eq!(result.u32(), 0x5000);
        let log = e.call_log.take().unwrap();
        let init: Vec<_> = log
            .iter()
            .filter(|call| call.0 == INITIALIZE_CRITICAL_SECTION_AND_SPIN_COUNT)
            .collect();
        assert_eq!(init.len(), 1);
        assert_eq!(init[0].1, vec![0x5000, 0xfa0]);
    }
}
