//! `(unplaced)/run_0084b720` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00438af0` (`thiscall`, no stack word): empties the map.
const EMPTY_MAP: u32 = 0x0043_8af0;
/// `00aa10f0` (`cdecl`, one word): frees a block.
const FREE_BLOCK: u32 = 0x00aa_10f0;

// Translated from 0084b720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): a map destructor. Sets the vtable
/// `0107f4c8`, empties the map (`00438af0`) and frees the bucket block at +8 (`00aa10f0`).
pub fn fn_0084b720(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0107_f4c8);
    e.call(EMPTY_MAP, &args![this]);
    let block = e.mem.u32(this.addr() + 8);
    e.call(FREE_BLOCK, &args![block]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0084_b720, fn_0084b720(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empties_and_frees_the_buckets() {
        let mut e = Engine::new();
        e.register(EMPTY_MAP, |_, _| Ret::default());
        e.register(FREE_BLOCK, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, 0x6000);
        e.call_log = Some(vec![]);
        e.call(0x0084_b720, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [(EMPTY_MAP, vec![this]), (FREE_BLOCK, vec![0x6000])]
        );
        assert_eq!(e.mem.u32(this), 0x0107_f4c8);
    }
}
