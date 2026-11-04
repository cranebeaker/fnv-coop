//! `fallout shared/run_004fd360` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004fd360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only, plain `RET`): `004fd420` with the mask 2; its
/// result is left in `EAX` and returned.
pub fn fn_004fd360(e: &mut Engine, this: Ptr) -> bool {
    e.call(0x004f_d420, &args![this, 2u32]).bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004f_d360, fn_004fd360(Ptr) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_004fd360_asks_for_bit_one() {
        for answer in [0u32, 1] {
            let mut e = Engine::new();
            e.register_double(0x004f_d420, move |_, _| Ret {
                eax: answer,
                ..Ret::default()
            });
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x004f_d360, &args![0x4000u32]).bool(), answer == 1);
            let log = e.call_log.take().unwrap();
            assert_eq!(log[1], (0x004f_d420, vec![0x4000, 2]));
        }
    }
}
