//! `fallout shared/run_0067ca10` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0067ca10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears five parts (`thiscall`, no stack words; the engine map gives it
/// no name): `00407a90` on +6, +8 and +0xA, then `00401080` on +0xC and
/// +0x10, each `cdecl` with a zero byte.
pub fn fn_0067ca10(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for offset in [6, 8, 0xa] {
        e.call(0x0040_7a90, &args![base + offset, 0u32]);
    }
    for offset in [0xc, 0x10] {
        e.call(0x0040_1080, &args![base + offset, 0u32]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0067_ca10, fn_0067ca10(Ptr))]
}

#[cfg(test)]
mod tests_n042 {
    use super::*;

    #[test]
    fn clears_five_parts() {
        let mut e = Engine::new();
        e.register(0x0040_7a90, |_, _| Ret::default());
        e.register(0x0040_1080, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0067_ca10, &args![0x5000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (0x0040_7a90, vec![0x5006, 0]),
                (0x0040_7a90, vec![0x5008, 0]),
                (0x0040_7a90, vec![0x500a, 0]),
                (0x0040_1080, vec![0x500c, 0]),
                (0x0040_1080, vec![0x5010, 0])
            ]
        );
    }
}
