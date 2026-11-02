//! `BSHavok/bhkcharacterstateclimbing.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `bhkCharacterController::UpdateState` (Xbox PDB, `00c6cba0`, `this` = the controller).
const UPDATE_STATE: u32 = 0x00c6_cba0;
/// The float 0.0 in the exe image (`01011d78`).
const ZERO_CONSTANT: u32 = 0x0101_1d78;

// Translated from 00cd3c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterStateClimbing::Update` (Xbox PDB), virtual `(this, controller)`; `this` is
/// not used. When the word at `controller + 0x520` is not 0xB it runs
/// `bhkCharacterController::UpdateState` (`00c6cba0`) on the controller. Otherwise it sets
/// bit 10 (0x400) of the flags at `+0x414` and stores the four words (0.0, 0.0, the word at
/// `+0x514`, 0.0) at `+0x4f0`; the zero is the float at `01011d78`.
pub fn bhk_character_state_climbing_update(e: &mut Engine, _this: Ptr, controller: Ptr) {
    let c = controller.addr();
    if e.mem.u32(c + 0x520) != 0xb {
        e.call(UPDATE_STATE, &args![controller]);
        return;
    }
    let zero = e.global::<u32>(ZERO_CONSTANT);
    let flags = e.mem.u32(c + 0x414);
    e.mem.set_u32(c + 0x414, flags | 0x400);
    let wanted = e.mem.u32(c + 0x514);
    e.mem.set_u32(c + 0x4f0, zero);
    e.mem.set_u32(c + 0x4f4, zero);
    e.mem.set_u32(c + 0x4f8, wanted);
    e.mem.set_u32(c + 0x4fc, zero);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00cd3c20,
        bhk_character_state_climbing_update(Ptr, Ptr)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_in_the_wanted_state_sets_the_flag_and_the_velocity() {
        let mut e = Engine::new();
        e.map(ZERO_CONSTANT, 4);
        e.set_global(ZERO_CONSTANT, 0u32);
        e.register(UPDATE_STATE, |_, _| Ret::default());
        let controller = e.mem.alloc(0x640);
        e.mem.set_u32(controller + 0x520, 0xb);
        e.mem.set_u32(controller + 0x414, 0x81);
        e.mem.set_u32(controller + 0x514, 0x3fc0_0000);
        for lane in 0..4u32 {
            e.mem
                .set_u32(controller + 0x4f0 + 4 * lane, 0xdead_0000 + lane);
        }
        e.call_log = Some(vec![]);
        e.call(0x00cd3c20, &args![0x1234u32, controller]);
        assert_eq!(e.mem.u32(controller + 0x414), 0x481);
        let lanes: Vec<u32> = (0..4u32)
            .map(|n| e.mem.u32(controller + 0x4f0 + 4 * n))
            .collect();
        assert_eq!(lanes, [0, 0, 0x3fc0_0000, 0]);
        let updates = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == UPDATE_STATE)
            .count();
        assert_eq!(updates, 0);
    }

    #[test]
    fn update_in_another_state_runs_the_controller_update_only() {
        let mut e = Engine::new();
        e.register(UPDATE_STATE, |_, _| Ret::default());
        let controller = e.mem.alloc(0x640);
        e.mem.set_u32(controller + 0x520, 3);
        e.mem.set_u32(controller + 0x414, 0x81);
        e.mem.set_u32(controller + 0x4f0, 0x77);
        e.call_log = Some(vec![]);
        e.call(0x00cd3c20, &args![0x1234u32, controller]);
        let log = e.call_log.as_ref().unwrap();
        let hits: Vec<_> = log.iter().filter(|c| c.0 == UPDATE_STATE).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, [controller]);
        assert_eq!(e.mem.u32(controller + 0x414), 0x81);
        assert_eq!(e.mem.u32(controller + 0x4f0), 0x77);
    }
}
