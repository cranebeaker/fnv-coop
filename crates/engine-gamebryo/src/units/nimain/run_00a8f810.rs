//! `NiMain/run_00a8f810` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::nimain::nimainsdm::{interlocked_decrement, release_fixed_string};

/// The vtables of `NiMaterial` and `NiRefObject` (their RTTI locators name
/// `.?AVNiMaterial@@` and `.?AVNiRefObject@@`).
const NI_MATERIAL_VTABLE: u32 = 0x0109_f73c;
const NI_REF_OBJECT_VTABLE: u32 = 0x0101_dce4;
/// `011f5e34`: the map of materials by name; `00aa0250` (unnamed in the engine map,
/// `thiscall` on the map, the address of a name handle) takes the name out of it.
const MATERIAL_MAP: u32 = 0x011f_5e34;
const MAP_REMOVE: u32 = 0x00aa_0250;
/// `011f4400`: the counted word `NiRefObject`'s destructor lowers.
const OBJECT_COUNT: u32 = 0x011f_4400;

// Translated from 00a8f810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): the `NiMaterial` destructor body
/// by the vtables it stores: the class's vtable, its name (+8) taken out of the material map
/// (`00aa0250` on the map at `011f5e34`), the name handle released (its count at -8,
/// unless it is the empty string's), then `NiRefObject`'s vtable and the object count at
/// `011f4400` lowered.
pub fn fn_00a8f810(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, NI_MATERIAL_VTABLE);
    let map = e.global::<u32>(MATERIAL_MAP);
    e.call(MAP_REMOVE, &args![map, base + 8]);
    let name = e.mem.u32(base + 8);
    release_fixed_string(e, name);
    e.mem.set_u32(base, NI_REF_OBJECT_VTABLE);
    interlocked_decrement(e, OBJECT_COUNT);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a8_f810, fn_00a8f810(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::{calls_to, engine_with_counting};

    #[test]
    fn removes_the_name_and_releases_it() {
        let mut e = engine_with_counting();
        e.register(MAP_REMOVE, |_, _| Ret::default());
        e.mem.set_u32(MATERIAL_MAP, 0x0500_0000);
        e.mem.set_u32(OBJECT_COUNT, 3);
        let text = e.mem.alloc(0x20);
        e.mem.set_u32(text, 2);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, text + 8);
        e.call_log = Some(vec![]);
        e.call(0x00a8_f810, &args![this]);
        assert_eq!(calls_to(&e, MAP_REMOVE), vec![vec![0x0500_0000, this + 8]]);
        assert_eq!(e.mem.u32(text), 1);
        assert_eq!(e.mem.u32(this), NI_REF_OBJECT_VTABLE);
        assert_eq!(e.mem.u32(OBJECT_COUNT), 2);
        // The empty name is not counted.
        e.mem.set_u32(this + 8, 0x0109_b228);
        e.call(0x00a8_f810, &args![this]);
        assert_eq!(e.mem.u32(OBJECT_COUNT), 1);
        assert_eq!(e.mem.u32(text), 1);
    }
}
