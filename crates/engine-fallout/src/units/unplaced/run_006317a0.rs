//! `(unplaced)/run_006317a0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006317a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 4`; the `this` register is saved and never read.
/// Answers the address of the item of a list node: the node's address plus 8 (no memory is
/// read).
pub fn fn_006317a0(_e: &mut Engine, _unused_this: Ptr, node: Ptr) -> u32 {
    node.addr().wrapping_add(8)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0063_17a0, fn_006317a0(Ptr, Ptr) -> u32)]
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    #[test]
    fn at_006317a0_answers_the_node_address_plus_8_without_reading_memory() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x0063_17a0, &args![0x1111u32, 0x4000u32]).u32(),
            0x4008
        );
        assert_eq!(e.call(0x0063_17a0, &args![0u32, 0u32]).u32(), 8);
        assert_eq!(e.call(0x0063_17a0, &args![0u32, 0xffff_fffcu32]).u32(), 4);
    }
}
