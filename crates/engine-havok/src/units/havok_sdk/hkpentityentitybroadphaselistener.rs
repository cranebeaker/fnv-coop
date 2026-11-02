//! `Havok SDK/hkpentityentitybroadphaselistener.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkAgentNnMachine_FindAgent` (Xbox PDB, `00cfdd70`, cdecl `(first collidable, second
/// collidable)`, answers the agent entry or 0) and `hkpWorldAgentUtil::removeAgent` (Xbox PDB,
/// `00cc10b0`, cdecl `(agent)`).
const FIND_AGENT: u32 = 0x00cf_dd70;
const REMOVE_AGENT: u32 = 0x00cc_10b0;

// Translated from 00d10860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpEntityEntityBroadPhaseListener::removeCollisionPair` (Xbox PDB), thiscall with one stack
/// word, a pair of broad-phase handles (two pointers, each handle giving its collidable at its
/// own address plus the signed byte at its +5): finds the agent between the two collidables
/// (`FindAgent`) and, if there is one, removes it (`removeAgent`). `this` is not read.
pub fn hkp_entity_entity_broad_phase_listener_remove_collision_pair(
    e: &mut Engine,
    _this: Ptr,
    pair: Ptr,
) {
    let pair = pair.addr();
    let first_handle = e.mem.u32(pair);
    let first = first_handle.wrapping_add(i32::from(e.mem.i8(first_handle + 5)) as u32);
    let second_handle = e.mem.u32(pair + 4);
    let second = second_handle.wrapping_add(i32::from(e.mem.i8(second_handle + 5)) as u32);
    let agent = e.call(FIND_AGENT, &args![first, second]).u32();
    if agent != 0 {
        e.call(REMOVE_AGENT, &args![agent]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00d10860,
        hkp_entity_entity_broad_phase_listener_remove_collision_pair(Ptr, Ptr)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removing_a_collision_pair_removes_the_agent_between_the_collidables() {
        let mut e = Engine::new();
        let mut handles = [0u32; 2];
        let mut collidables = [0u32; 2];
        for i in 0..2 {
            let block = e.mem.alloc(0x40);
            e.mem.set_i8(block + 5, 12);
            handles[i] = block;
            collidables[i] = block + 12;
        }
        let pair = e.mem.alloc(8);
        e.mem.set_u32(pair, handles[0]);
        e.mem.set_u32(pair + 4, handles[1]);
        e.register(FIND_AGENT, |_, _| 0xa6e7u32.into_ret());
        e.register(REMOVE_AGENT, |_, _| Ret::default());
        e.call_log = Some(Vec::new());
        e.call(0x00d1_0860, &args![0u32, pair]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (FIND_AGENT, vec![collidables[0], collidables[1]]),
                (REMOVE_AGENT, vec![0xa6e7])
            ]
        );
    }

    #[test]
    fn removing_a_collision_pair_without_an_agent_does_nothing_more() {
        let mut e = Engine::new();
        let handle = e.mem.alloc(0x40);
        let pair = e.mem.alloc(8);
        e.mem.set_u32(pair, handle);
        e.mem.set_u32(pair + 4, handle);
        e.register(FIND_AGENT, |_, _| 0u32.into_ret());
        e.register(REMOVE_AGENT, |_, _| panic!("no agent"));
        e.call(0x00d1_0860, &args![0u32, pair]);
    }
}
