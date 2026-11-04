//! `fallout shared/run_00594d60` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00594d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The form-parameter check of script function 0x198 (unnamed in the map;
/// `cdecl`, two words; `TESParameters` reaches it through `006822a0`):
/// true only when the parameter index is 1 and the word at `+4` of the
/// function data (the first argument) is 0, 1, 2, 3, 9 or 10 (the exe's jump table at
/// `00594da4` with its index bytes at `00594dac`, values 0 to 10); every
/// other value, and every other index, answers false.
pub fn fn_00594d60(e: &mut Engine, function_data: Ptr, index: u32) -> bool {
    if index != 1 {
        return false;
    }
    matches!(e.mem.u32(function_data.addr() + 4), 0..=3 | 9 | 10)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00594d60, fn_00594d60(Ptr, u32) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_the_listed_kinds_only_for_the_second_parameter() {
        let mut e = Engine::new();
        let parameter = e.mem.alloc(8);
        let check = |e: &mut Engine, kind: u32, index: u32| {
            e.mem.set_u32(parameter + 4, kind);
            e.call(0x0059_4d60, &args![Ptr::<()>::new(parameter), index])
                .bool()
        };
        for kind in 0..=12u32 {
            let expected = matches!(kind, 0 | 1 | 2 | 3 | 9 | 10);
            assert_eq!(check(&mut e, kind, 1), expected, "kind {kind}");
            assert!(!check(&mut e, kind, 0), "kind {kind} index 0");
            assert!(!check(&mut e, kind, 2), "kind {kind} index 2");
        }
        assert!(!check(&mut e, 0xffff_ffff, 1));
    }
}
