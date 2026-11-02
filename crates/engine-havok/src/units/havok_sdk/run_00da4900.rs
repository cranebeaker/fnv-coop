//! `Havok SDK/run_00da4900` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkAgent1nMachine_VisitAllAgents` (`00da44d0`, cdecl `(track, input, callback)`).
const VISIT_ALL_AGENTS: u32 = 0x00da_44d0;
/// The callback `00da40e0` this function visits the agents with.
const VISIT_CALLBACK: u32 = 0x00da_40e0;

// Translated from 00da4900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Visits all agents of `track` with `input` and the callback `00da40e0`: cdecl `(track, input)`
/// (the sibling of `hkAgent1nMachine_UpdateShapeCollectionFilter`, `00da48e0`, which uses the
/// callback `00da3f90`). The exe's name for it is unknown.
pub fn fn_00da4900(e: &mut Engine, track: u32, input: u32) {
    e.call(VISIT_ALL_AGENTS, &args![track, input, VISIT_CALLBACK]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00da_4900, fn_00da4900(u32, u32))]
}

#[cfg(test)]
mod a09_tests {
    use super::*;

    #[test]
    fn the_agents_are_visited_with_the_callback() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(VISIT_ALL_AGENTS, |_, _| Ret::default());
        e.call(0x00da_4900, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.as_ref().unwrap();
        assert_eq!(
            log.iter()
                .filter(|(a, _)| *a == VISIT_ALL_AGENTS)
                .collect::<Vec<_>>(),
            vec![&(VISIT_ALL_AGENTS, vec![0x1000, 0x2000, 0x00da_40e0])]
        );
    }
}
