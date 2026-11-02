//! `fallout shared/run_0048cfe0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0048cfe0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0048cfe0 - the exception-unwinding frame (FS:[0]) and its state numbers are not translated
/// `TESSpellList`'s constructor (vtable `0101d628`, RTTI `TESSpellList`):
/// the `BaseFormComponent` base (`00402d30`), the vtable, the two lists at
/// +4 and +0xC (`0096a2d0`) and `00483710` on `this`. Returns `this`.
pub fn fn_0048cfe0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0040_2d30, &args![this]);
    e.mem.set_u32(this.addr(), 0x0101_d628);
    e.call(0x0096_a2d0, &args![this.addr() + 4]);
    e.call(0x0096_a2d0, &args![this.addr() + 0xc]);
    e.call(0x0048_3710, &args![this]);
    this
}

// Translated from 0048d060 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0048d060 - the exception-unwinding frame (FS:[0]) and its state numbers are not translated
/// `TESSpellList`'s destructor: the vtable, `ClearDataComponent`
/// (`TESSpellList::ClearDataComponent`, `0048d0d0`), then the lists at +0xC
/// and +4 (`0046ffb0`).
pub fn fn_0048d060(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0101_d628);
    e.call(0x0048_d0d0, &args![this]);
    e.call(0x0046_ffb0, &args![this.addr() + 0xc]);
    e.call(0x0046_ffb0, &args![this.addr() + 4]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0048_cfe0, fn_0048cfe0(Ptr) -> Ptr),
        entry!(0x0048_d060, fn_0048d060(Ptr)),
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
    fn b020_0048cfe0_constructs() {
        let mut e = Engine::new();
        for a in [0x0040_2d30, 0x0096_a2d0, 0x0048_3710] {
            e.register(a, |_, _| Ret::default());
        }
        let p = e.mem.alloc(0x14);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0048_cfe0, &args![p]).u32(), p);
        assert_eq!(e.mem.u32(p), 0x0101_d628);
        assert_eq!(
            calls(&mut e, 0x0048_cfe0),
            vec![
                (0x0040_2d30, vec![p]),
                (0x0096_a2d0, vec![p + 4]),
                (0x0096_a2d0, vec![p + 0xc]),
                (0x0048_3710, vec![p]),
            ]
        );
    }

    #[test]
    fn b020_0048d060_destroys() {
        let mut e = Engine::new();
        for a in [0x0048_d0d0, 0x0046_ffb0] {
            e.register(a, |_, _| Ret::default());
        }
        let p = e.mem.alloc(0x14);
        e.call_log = Some(vec![]);
        e.call(0x0048_d060, &args![p]);
        assert_eq!(e.mem.u32(p), 0x0101_d628);
        assert_eq!(
            calls(&mut e, 0x0048_d060),
            vec![
                (0x0048_d0d0, vec![p]),
                (0x0046_ffb0, vec![p + 0xc]),
                (0x0046_ffb0, vec![p + 4]),
            ]
        );
    }
}
