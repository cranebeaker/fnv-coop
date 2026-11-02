//! `NiXenonRenderer/nid3drenderstategroup.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiD3DRenderStateGroup::ms_pkD3DRenderState` (Xbox PDB static): the
/// `NiXenonRenderState`.
const RENDER_STATE: u32 = 0x0126_f714;
/// `m_pkSavedRenderStates` (Xbox PDB): the first saved entry
/// (`NiD3DRSEntry`: state +0, next +8).
const SAVED_RENDER_STATES: u32 = 0x10;
/// The render state's vtable slot that sets a state (+0x68, `(state, value, 0, 0)`).
const RENDER_STATE_SLOT_SET: u32 = 0x68;
/// The render state's table of saved values (8 bytes per state, value first).
const SAVED_VALUES: u32 = 0x124;

// Translated from 00e7ef40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DRenderStateGroup::RestoreRenderStates` (Xbox PDB): for each saved
/// entry, sets its state back to the render state's saved value for it
/// (`ms_pkD3DRenderState`, read for each entry). Answers 0.
pub fn ni_d3d_render_state_group_restore_render_states(e: &mut Engine, this: Ptr) -> u32 {
    let mut entry = e.mem.u32(this.addr() + SAVED_RENDER_STATES);
    while entry != 0 {
        let state = e.mem.u32(entry);
        let render_state: u32 = e.global(RENDER_STATE);
        let value = e.mem.u32(
            render_state
                .wrapping_add(state.wrapping_mul(8))
                .wrapping_add(SAVED_VALUES),
        );
        e.vcall(
            render_state,
            RENDER_STATE_SLOT_SET,
            &args![state, value, 0u32, 0u32],
        );
        entry = e.mem.u32(entry + 8);
    }
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00e7ef40,
        ni_d3d_render_state_group_restore_render_states(Ptr) -> u32
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    const SET_DOUBLE: u32 = 0xdead_6000;

    #[test]
    fn restores_each_saved_state() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        let vtable = e.mem.alloc(0x70);
        e.mem.set_u32(vtable + RENDER_STATE_SLOT_SET, SET_DOUBLE);
        let render_state = e.mem.alloc(0x200);
        e.mem.set_u32(render_state, vtable);
        e.mem.set_u32(render_state + SAVED_VALUES + 8 * 3, 0x33);
        e.mem.set_u32(render_state + SAVED_VALUES + 8 * 7, 0x77);
        e.set_global(RENDER_STATE, render_state);
        e.register(SET_DOUBLE, |_, _| Ret::default());
        let second = e.mem.alloc(0xc);
        e.mem.set_u32(second, 7);
        let first = e.mem.alloc(0xc);
        e.mem.set_u32(first, 3);
        e.mem.set_u32(first + 8, second);
        let group = e.mem.alloc(0x14);
        e.mem.set_u32(group + SAVED_RENDER_STATES, first);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e7_ef40, &args![group]).u32(), 0);
        let log = e.call_log.take().unwrap();
        let sets: Vec<_> = log
            .iter()
            .filter(|c| c.0 == SET_DOUBLE)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(
            sets,
            vec![
                vec![render_state, 3, 0x33, 0, 0],
                vec![render_state, 7, 0x77, 0, 0]
            ]
        );
    }
}
