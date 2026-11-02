//! `fallout shared/sky/run_006366b0` (Xbox PDB source unit), subsystem `fallout shared/sky`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// thiscall (object, mask): 1 when the word at +0x30 has any bit of the
/// mask, else 0 (`00456630`).
const FLAG_TEST: u32 = 0x0045_6630;

// Translated from 006366b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: tests the flag bit 0x20 of `this` (the
/// object's word at +0x30) through `00456630` and answers its result
/// unchanged (thiscall, no stack words).
pub fn fn_006366b0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(FLAG_TEST, &args![this, 0x20u32]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006366b0, fn_006366b0(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_bit_0x20_through_the_flag_test() {
        let mut e = Engine::new();
        e.register(FLAG_TEST, |e, a| Ret {
            eax: u32::from(e.mem.u32(a[0] + 0x30) & a[1] != 0),
            ..Ret::default()
        });
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object + 0x30, 0x20);
        assert_eq!(e.call(0x0063_66b0, &args![object]).u32(), 1);
        e.mem.set_u32(object + 0x30, 0x10);
        assert_eq!(e.call(0x0063_66b0, &args![object]).u32(), 0);
    }
}
