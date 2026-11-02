//! `(unplaced)/run_00e91790` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `NiRTTI` record `00e91740` answers (`0126f9f8`), which the cast below looks for.
const OWN_RTTI: u32 = 0x0126_f9f8;
/// `NiPersistentSrcTextureRendererData::IsEqual` (engine map, `00a982f0`; thiscall: the
/// other object).
const BASE_IS_EQUAL: u32 = 0x00a9_82f0;

// Translated from 00e91790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a vtable word at `010f09f8`): an `IsEqual`. False unless
/// `other` is not null and its RTTI chain (vtable +8, then each record's base at +4)
/// reaches the record at `0126f9f8`; then whether the base `IsEqual` (`00a982f0`) answers a
/// non-zero byte.
pub fn fn_00e91790(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let other = other.addr();
    if other == 0 {
        return false;
    }
    let mut rtti = e.vcall(other, 0x8, &args![]).u32();
    loop {
        if rtti == 0 {
            return false;
        }
        if rtti == OWN_RTTI {
            break;
        }
        rtti = e.mem.u32(rtti + 4);
    }
    e.call(BASE_IS_EQUAL, &args![this, other]).u8() != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e9_1790, fn_00e91790(Ptr, Ptr) -> bool)]
}

#[cfg(test)]
mod n081_tests {
    use super::*;

    #[test]
    fn equal_only_for_the_class_and_a_true_base_answer() {
        for (kind, base, want) in [
            (true, 0x100u32, false),
            (true, 0x101, true),
            (false, 0x101, false),
        ] {
            let mut e = Engine::new();
            let leaf = e.mem.alloc(8);
            e.mem.set_u32(leaf + 4, if kind { OWN_RTTI } else { 0 });
            e.register_double(0x0079_0008, move |_, _| leaf.into_ret());
            e.register_double(BASE_IS_EQUAL, move |_, _| base.into_ret());
            let vtable = e.mem.alloc(0xc);
            e.mem.set_u32(vtable + 8, 0x0079_0008);
            let other = e.mem.alloc(4);
            e.mem.set_u32(other, vtable);
            assert_eq!(e.call(0x00e9_1790, &args![0x5000u32, other]).bool(), want);
        }
        let mut e = Engine::new();
        assert!(!e.call(0x00e9_1790, &args![0x5000u32, 0u32]).bool());
    }
}
