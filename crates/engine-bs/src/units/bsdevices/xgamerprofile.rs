//! `BSDevices/xgamerprofile.obj` (Xbox PDB source unit), subsystem `BSDevices`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `008256d0` (`thiscall`, no stack words, the answer in `EAX`).
const FN_008256D0: u32 = 0x0082_56d0;

// Translated from 007d6e70 (disassembled, FalloutNV.exe 1.4.0.525)
/// `XGamerProfile::Instance` (engine map; `thiscall`, no stack words, plain `RET`): `008256d0` on
/// the object's +4 (`ADD ECX,4`), whose answer is left in `EAX`.
pub fn xgamer_profile_instance(e: &mut Engine, this: Ptr) -> u32 {
    e.call(FN_008256D0, &args![this.addr().wrapping_add(4)])
        .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x007d_6e70, xgamer_profile_instance(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instance_runs_008256d0_on_the_object_plus_4() {
        let mut e = Engine::new();
        e.register(FN_008256D0, |_, a| (a[0] ^ 0xff).into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007d_6e70, &args![0x1000u32]).u32(), 0x1004 ^ 0xff);
        let calls: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == FN_008256D0)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(calls, vec![vec![0x1004]]);
    }
}
