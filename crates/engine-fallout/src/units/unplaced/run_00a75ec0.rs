//! `(unplaced)/run_00a75ec0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The game's `operator new` (`cdecl`, the size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// `NiTriBasedGeomData::NiTriBasedGeomData` (engine map, `00a8a970`; `thiscall`).
const TRI_BASED_GEOM_DATA_CONSTRUCT: u32 = 0x00a8_a970;
/// The `NiTriStripsData` vtable (RTTI).
const TRI_STRIPS_DATA_VTABLE: u32 = 0x0109_d8ac;

// Translated from 00a75ec0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function, no engine-map name; by its body the creation function of
/// `NiTriStripsData`, the RTTI of the vtable it stores; no arguments): a new 0x50-byte
/// block from `operator new`; null when that fails. Otherwise
/// `NiTriBasedGeomData::NiTriBasedGeomData`, the 16-bit word at +0x44 and the words at
/// +0x48 and +0x4C zeroed, and the `NiTriStripsData` vtable. Answers the block.
pub fn fn_00a75ec0(e: &mut Engine) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x50u32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(TRI_BASED_GEOM_DATA_CONSTRUCT, &args![block]);
    e.mem.set_u16(block + 0x44, 0);
    e.mem.set_u32(block + 0x48, 0);
    e.mem.set_u32(block + 0x4c, 0);
    e.mem.set_u32(block, TRI_STRIPS_DATA_VTABLE);
    block
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a7_5ec0, fn_00a75ec0() -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00a75ec0_builds_tri_strips_data() {
        let mut e = Engine::new();
        let block = e.mem.alloc(0x50);
        for offset in (0..0x50).step_by(4) {
            e.mem.set_u32(block + offset, 0xdead_beef);
        }
        e.register_double(OPERATOR_NEW, move |_, _| block.into_ret());
        e.register(TRI_BASED_GEOM_DATA_CONSTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a7_5ec0, &[]).u32(), block);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(OPERATOR_NEW, vec![0x50])));
        assert!(log.contains(&(TRI_BASED_GEOM_DATA_CONSTRUCT, vec![block])));
        assert_eq!(e.mem.u32(block), TRI_STRIPS_DATA_VTABLE);
        assert_eq!(e.mem.u32(block + 0x44), 0xdead_0000);
        assert_eq!(e.mem.u32(block + 0x48), 0);
        assert_eq!(e.mem.u32(block + 0x4c), 0);

        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a7_5ec0, &[]).u32(), 0);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }
}
