//! `fallout shared/run_0048ba00` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0048ba00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0048ba00 - the exception-unwinding frame (FS:[0]) and its state numbers are not translated
/// `TESReactionForm`'s constructor: the `BaseFormComponent` base
/// (`00402d30`), the vtable `0101d46c` (RTTI `TESReactionForm`), the
/// reaction list at +4 (`0096a2d0`) and `InitializeDataComponent`
/// (`0048bad0`). Returns `this`.
pub fn fn_0048ba00(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0040_2d30, &args![this]);
    e.mem.set_u32(this.addr(), 0x0101_d46c);
    e.call(0x0096_a2d0, &args![this.addr() + 4]);
    e.call(0x0048_bad0, &args![this]);
    this
}

// Translated from 0048ba70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0048ba70 - the exception-unwinding frame (FS:[0]) and its state numbers are not translated
/// `TESReactionForm`'s destructor: the vtable `0101d46c`,
/// `ClearDataComponent` (`0048baf0`), then the reaction list at +4
/// (`0046ffb0`).
pub fn fn_0048ba70(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0101_d46c);
    e.call(0x0048_baf0, &args![this]);
    e.call(0x0046_ffb0, &args![this.addr() + 4]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0048_ba00, fn_0048ba00(Ptr) -> Ptr),
        entry!(0x0048_ba70, fn_0048ba70(Ptr)),
    ]
}

#[cfg(test)]
mod b020_tests {
    use super::*;

    fn calls(e: &mut Engine, skip: u32) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 != skip)
            .collect()
    }

    #[test]
    fn b020_0048ba00_constructs() {
        let mut e = Engine::new();
        for a in [0x0040_2d30, 0x0096_a2d0, 0x0048_bad0] {
            e.register(a, |_, _| Ret::default());
        }
        let p = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0048_ba00, &args![p]).u32(), p);
        assert_eq!(e.mem.u32(p), 0x0101_d46c);
        assert_eq!(
            calls(&mut e, 0x0048_ba00),
            vec![
                (0x0040_2d30, vec![p]),
                (0x0096_a2d0, vec![p + 4]),
                (0x0048_bad0, vec![p]),
            ]
        );
    }

    #[test]
    fn b020_0048ba70_destroys() {
        let mut e = Engine::new();
        for a in [0x0048_baf0, 0x0046_ffb0] {
            e.register(a, |_, _| Ret::default());
        }
        let p = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x0048_ba70, &args![p]);
        assert_eq!(e.mem.u32(p), 0x0101_d46c);
        assert_eq!(
            calls(&mut e, 0x0048_ba70),
            vec![(0x0048_baf0, vec![p]), (0x0046_ffb0, vec![p + 4])]
        );
    }
}
