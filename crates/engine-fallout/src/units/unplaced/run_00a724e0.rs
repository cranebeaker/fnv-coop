//! `(unplaced)/run_00a724e0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiAdditionalGeometryData::NiAGDDataBlock`'s vtable (its class confirmed by the RTTI
/// before it), the `MemoryManager` singleton and its `Deallocate` (`thiscall` on it,
/// `(block)`).
const AGD_DATA_BLOCK_VTABLE: u32 = 0x0109_cf58;
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;

// Translated from 00a724e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor in `NiAdditionalGeometryData::NiAGDDataBlock`'s vtable
/// (unnamed in the map): the class's vtable, then, when bit 0 of `flags` is set, the block
/// is given back to `MemoryManager::Deallocate`. Answers `this`.
pub fn fn_00a724e0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), AGD_DATA_BLOCK_VTABLE);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a724e0, fn_00a724e0(Ptr, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00a724e0_frees_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        let this = e.mem.alloc(8);
        assert_eq!(e.call(0x00a724e0, &args![this, 2u32]).u32(), this);
        assert_eq!(e.mem.u32(this), AGD_DATA_BLOCK_VTABLE);
        e.call(0x00a724e0, &args![this, 1u32]);
        let frees: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == MEMORY_DEALLOCATE)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(frees, vec![vec![MEMORY_MANAGER, this]]);
    }
}
