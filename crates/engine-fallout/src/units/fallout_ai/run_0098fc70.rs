//! `fallout/ai/run_0098fc70` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0098fc70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0098fc70 - the compiler's exception frame (FS:[0]) is not translated
/// Constructs `count` 0x1C-byte entries at `entries` (placement `006e6da0`,
/// then `0098fda0` on each non-null one; `RET 8`).
pub fn fn_0098fc70(e: &mut Engine, _this: Ptr, entries: u32, count: u32) {
    let mut index = 0u32;
    while index < count {
        let at = index.wrapping_mul(0x1c).wrapping_add(entries);
        let block = e.call(0x006e_6da0, &args![0x1cu32, at]).u32();
        if block != 0 {
            e.call(0x0098_fda0, &args![block]);
        }
        index += 1;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0098_fc70, fn_0098fc70(Ptr, u32, u32))]
}

#[cfg(test)]
mod n028_tests {
    use super::*;

    #[test]
    fn fn_0098fc70_constructs_each_entry() {
        let mut e = Engine::new();
        e.register(0x006e_6da0, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        e.register(0x0098_fda0, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0098_fc70, &[0, 0x1000, 2]);
        let built: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == 0x0098_fda0)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(built, vec![vec![0x1000], vec![0x101c]]);
    }
}
