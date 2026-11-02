//! `NiParticle/run_00c21910` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c1af20` (unnamed, `NiParticle/niparticlesystem.cpp`, `thiscall`, two
/// words): the particle system's update step this slot wraps.
const SYSTEM_UPDATE_STEP: u32 = 0x00c1_af20;

// Translated from 00c21910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; a virtual slot (its address is stored at
/// `010bde2c` and `010bde30`), `thiscall`, `RET 8`. When the byte at +5 of
/// `update` is set it only sets the byte at +6 to 1. Otherwise it runs
/// `00c1af20` with both words and copies the float at `update` +0 to `this`
/// +0x110 (`FLD`/`FSTP`, quieting a signalling NaN).
pub fn fn_00c21910(e: &mut Engine, this: Ptr, update: Ptr, value: u32) {
    if e.mem.u8(update.addr() + 5) != 0 {
        e.mem.set_u8(update.addr() + 6, 1);
        return;
    }
    e.call(SYSTEM_UPDATE_STEP, &args![this, update, value]);
    let time = x87_quiet_bits(e.mem.u32(update.addr()));
    e.mem.set_u32(this.addr() + 0x110, time);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c2_1910, fn_00c21910(Ptr, Ptr, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00c21910_marks_a_deferred_update_or_runs_it_and_keeps_the_time() {
        let mut e = Engine::new();
        e.register(SYSTEM_UPDATE_STEP, |_, _| Ret::default());
        let this = e.mem.alloc(0x120);
        let update = e.mem.alloc(8);
        e.mem.set_u32(update, 0x7f80_0003);
        e.mem.set_u8(update + 5, 1);
        e.call_log = Some(vec![]);
        e.call(0x00c2_1910, &args![this, update, 9u32]);
        assert_eq!(e.mem.u8(update + 6), 1);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        e.mem.set_u8(update + 5, 0);
        e.call(0x00c2_1910, &args![this, update, 9u32]);
        let log = e.call_log.take().unwrap();
        let steps: Vec<_> = log.iter().filter(|c| c.0 == SYSTEM_UPDATE_STEP).collect();
        assert_eq!(steps.len(), 1);
        assert_eq!(steps[0].1, vec![this, update, 9]);
        assert_eq!(e.mem.u32(this + 0x110), 0x7fc0_0003);
    }
}
