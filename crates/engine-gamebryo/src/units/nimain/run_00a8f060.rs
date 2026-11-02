//! `NiMain/run_00a8f060` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00aa10f0` (cdecl, one word): the Gamebryo free (`NiFree`, as other units
/// call it).
const NI_FREE: u32 = 0x00aa_10f0;

// Translated from 00a8f060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): frees the block at
/// `this+0x50` through `00aa10f0` and clears the four words `+0x50..+0x5c`
/// (stored in the order `+0x5c`, `+0x58`, `+0x54`, `+0x50`).
pub fn fn_00a8f060(e: &mut Engine, this: Ptr) {
    let block = e.mem.u32(this.addr() + 0x50);
    e.call(NI_FREE, &args![block]);
    e.mem.set_u32(this.addr() + 0x5c, 0);
    e.mem.set_u32(this.addr() + 0x58, 0);
    e.mem.set_u32(this.addr() + 0x54, 0);
    e.mem.set_u32(this.addr() + 0x50, 0);
}

// Translated from 00a8f080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two words; `RET 8`): stores them at +8 and +0xc.
pub fn fn_00a8f080(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.mem.set_u32(this.addr() + 8, first);
    e.mem.set_u32(this.addr() + 0xc, second);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a8_f060, fn_00a8f060(Ptr)),
        entry!(0x00a8_f080, fn_00a8f080(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00a8f060_frees_the_block_and_clears_four_words() {
        let mut e = Engine::new();
        e.register(NI_FREE, |_, _| Ret::default());
        let this = e.mem.alloc(0x60);
        for i in 0..4 {
            e.mem.set_u32(this + 0x50 + 4 * i, 0x5000 + i);
        }
        e.mem.set_u32(this + 0x4c, 9);
        e.call_log = Some(vec![]);
        e.call(0x00a8_f060, &args![this]);
        let log = e.call_log.take().unwrap();
        let frees: Vec<_> = log.iter().filter(|c| c.0 == NI_FREE).collect();
        assert_eq!(frees.len(), 1);
        assert_eq!(frees[0].1, vec![0x5000]);
        for i in 0..4 {
            assert_eq!(e.mem.u32(this + 0x50 + 4 * i), 0);
        }
        assert_eq!(e.mem.u32(this + 0x4c), 9);
    }
}

#[cfg(test)]
mod store_tests {
    use super::*;

    #[test]
    fn stores_both_words() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.call(0x00a8_f080, &args![this, 0x11u32, 0x22u32]);
        assert_eq!(e.mem.u32(this + 8), 0x11);
        assert_eq!(e.mem.u32(this + 0xc), 0x22);
        assert_eq!(e.mem.u32(this + 4), 0);
    }
}
