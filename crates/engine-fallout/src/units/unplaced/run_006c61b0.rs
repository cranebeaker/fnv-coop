//! `(unplaced)/run_006c61b0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00761540` (thiscall): the index of a free slot in the array.
const FREE_SLOT_INDEX: u32 = 0x0076_1540;
/// `00822780` (thiscall, 2 words: slot address, count).
const PREPARE_SLOT: u32 = 0x0082_2780;
/// `006e5cc0` (thiscall, one word): `NiPointer` assignment (the engine map
/// names this instance `NiPointer<PathingDebugGeometryData>::operator=`).
const NI_POINTER_ASSIGN: u32 = 0x006e_5cc0;

// Translated from 006c61b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: adds `object` to an array of `NiPointer`s
/// (data at `this + 4`): takes a slot index (`00761540`), prepares the slot
/// (`00822780(slot, 1)`), assigns `object` to it (`006e5cc0`) and returns
/// the index.
pub fn fn_006c61b0(e: &mut Engine, this: Ptr, object: u32) -> u32 {
    let t = this.addr();
    let index = e.call(FREE_SLOT_INDEX, &args![this]).u32();
    let slot = e
        .mem
        .u32(t.wrapping_add(4))
        .wrapping_add(index.wrapping_mul(4));
    e.call(PREPARE_SLOT, &args![this, slot, 1u32]);
    let slot = e
        .mem
        .u32(t.wrapping_add(4))
        .wrapping_add(index.wrapping_mul(4));
    e.call(NI_POINTER_ASSIGN, &args![slot, object]);
    index
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006c_61b0, fn_006c61b0(Ptr, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_006c61b0_fills_the_free_slot_and_returns_its_index() {
        let mut e = Engine::new();
        e.register(FREE_SLOT_INDEX, |_, _| Ret {
            eax: 3,
            ..Ret::default()
        });
        e.register(PREPARE_SLOT, |_, _| Ret::default());
        e.register(NI_POINTER_ASSIGN, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0x5000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006c_61b0, &args![this, 0x7777u32]).u32(), 3);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (FREE_SLOT_INDEX, vec![this]));
        assert_eq!(log[2], (PREPARE_SLOT, vec![this, 0x500c, 1]));
        assert_eq!(log[3], (NI_POINTER_ASSIGN, vec![0x500c, 0x7777]));
    }
}
