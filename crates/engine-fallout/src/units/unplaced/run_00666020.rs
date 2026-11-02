//! `(unplaced)/run_00666020` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00438af0` (`thiscall`, no stack word).
const EMPTY_LIST: u32 = 0x0043_8af0;
/// `00aa10f0` (`cdecl`, one word): frees a block.
const FREE_BLOCK: u32 = 0x00aa_10f0;

// Translated from 00666020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): a destructor. Sets the vtable
/// `010664c4`, calls `00438af0` on `this` and frees the block at +8 (`00aa10f0`).
pub fn fn_00666020(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0106_64c4);
    e.call(EMPTY_LIST, &args![this]);
    let block = e.mem.u32(this.addr() + 8);
    e.call(FREE_BLOCK, &args![block]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0066_6020, fn_00666020(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empties_then_frees_the_block() {
        let mut e = Engine::new();
        e.register(EMPTY_LIST, |_, _| Ret::default());
        e.register(FREE_BLOCK, |_, _| Ret::default());
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this + 8, 0x6000);
        e.call_log = Some(vec![]);
        e.call(0x0066_6020, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [(EMPTY_LIST, vec![this]), (FREE_BLOCK, vec![0x6000])]
        );
        assert_eq!(e.mem.u32(this), 0x0106_64c4);
    }
}
