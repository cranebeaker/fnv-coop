//! `fallout shared/run_0048b560` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00402d30` (`thiscall`): the `BaseFormComponent` base constructor.
const BASE_COMPONENT_CONSTRUCT: u32 = 0x0040_2d30;
/// `TESProduceForm`'s vtable (`0101d2a4`; RTTI `TESProduceForm`, read
/// through its locator).
const TES_PRODUCE_FORM_VTABLE: u32 = 0x0101_d2a4;
/// `TESProduceForm::InitializeDataComponent` (engine map, `0048b590`,
/// `thiscall`).
const INITIALIZE_DATA: u32 = 0x0048_b590;

// Translated from 0048b560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESProduceForm`'s constructor (the class from the vtable's RTTI; the
/// engine map gives it no name; no stack word): the base constructor
/// (`00402d30`), the vtable `0101d2a4`, then `InitializeDataComponent`
/// (`0048b590`). Returns `this`.
pub fn fn_0048b560(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_COMPONENT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), TES_PRODUCE_FORM_VTABLE);
    e.call(INITIALIZE_DATA, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0048_b560, fn_0048b560(Ptr) -> Ptr)]
}

#[cfg(test)]
mod n038_tests {
    use super::*;

    #[test]
    fn at_0048b560_constructs() {
        let mut e = Engine::new();
        e.register(BASE_COMPONENT_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            Ret::default()
        });
        // The vtable is in place when the initializer runs.
        e.register(INITIALIZE_DATA, |e, a| {
            assert_eq!(e.mem.u32(a[0]), TES_PRODUCE_FORM_VTABLE);
            Ret::default()
        });
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0048_b560, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (BASE_COMPONENT_CONSTRUCT, vec![this]),
                (INITIALIZE_DATA, vec![this])
            ]
        );
        assert_eq!(e.mem.u32(this), TES_PRODUCE_FORM_VTABLE);
    }
}
