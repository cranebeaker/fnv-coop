//! `(unplaced)/run_004edcc0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of `NiTArray<NiTexturingProperty::FilterMode, NiTMallocInterface<...>>` (its
/// class confirmed by the RTTI before it) and the array free `004ede70` (`cdecl`,
/// `(block)`).
const FILTER_MODE_ARRAY_VTABLE: u32 = 0x0102_32c4;
const ARRAY_FREE_004EDE70: u32 = 0x004e_de70;

// Translated from 004edcc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTArray<NiTexturingProperty::FilterMode, NiTMallocInterface<...>>` destructor body
/// (unnamed in the map): the class's vtable and the element block (`+4`) freed
/// (`004ede70`).
pub fn fn_004edcc0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), FILTER_MODE_ARRAY_VTABLE);
    let elements = e.mem.u32(this.addr() + 4);
    e.call(ARRAY_FREE_004EDE70, &args![elements]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004edcc0, fn_004edcc0(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_mode_array_destructor_frees_the_elements() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(ARRAY_FREE_004EDE70, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0x7100);
        e.call(0x004edcc0, &args![this]);
        assert_eq!(e.mem.u32(this), FILTER_MODE_ARRAY_VTABLE);
        let frees = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == ARRAY_FREE_004EDE70)
            .count();
        assert_eq!(frees, 1);
    }
}
