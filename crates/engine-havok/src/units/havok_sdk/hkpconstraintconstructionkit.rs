//! `Havok SDK/hkpconstraintconstructionkit.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d51720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintConstructionKit::begin` (Xbox PDB): `this` is the kit (ECX), `data` an
/// `hkpGenericConstraintData`. Stores `data` at +0 and its scheme (`getScheme`, `00d504f0`,
/// run with `data` as `this`) at +4, clears the words at +8 and +0xc, and clears the ten flag
/// bytes at +0x10 to +0x19.
pub fn hkp_constraint_construction_kit_begin(e: &mut Engine, this: Ptr, data: Ptr) {
    let scheme = e.call(0x00d5_04f0, &args![data]).u32();
    let at = this.addr();
    e.mem.set_u32(at + 4, scheme);
    e.mem.set_u32(at, data.addr());
    e.mem.set_u32(at + 8, 0);
    e.mem.set_u32(at + 0xc, 0);
    for offset in [0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x18, 0x19, 0x16, 0x17] {
        e.mem.set_u8(at + offset, 0);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00d5_1720,
        hkp_constraint_construction_kit_begin(Ptr, Ptr)
    )]
}

#[cfg(test)]
mod havok_a03_tests {
    use super::*;

    #[test]
    fn begin_stores_the_data_and_its_scheme_and_clears_the_rest() {
        let mut e = Engine::new();
        e.register(0x00d5_04f0, |_, a| Ret {
            eax: a[0] + 0x1000,
            ..Ret::default()
        });
        let data = e.mem.alloc(8);
        let kit = e.mem.alloc(0x20);
        for offset in (0..0x20).step_by(4) {
            e.mem.set_u32(kit + offset, 0xffff_ffff);
        }
        e.call(0x00d5_1720, &args![kit, data]);
        assert_eq!(e.mem.u32(kit), data);
        assert_eq!(e.mem.u32(kit + 4), data + 0x1000);
        assert_eq!(e.mem.u32(kit + 8), 0);
        assert_eq!(e.mem.u32(kit + 0xc), 0);
        for offset in 0x10..0x1a {
            assert_eq!(e.mem.u8(kit + offset), 0);
        }
        // Bytes past the ten flags are left alone.
        assert_eq!(e.mem.u8(kit + 0x1a), 0xff);
    }
}
