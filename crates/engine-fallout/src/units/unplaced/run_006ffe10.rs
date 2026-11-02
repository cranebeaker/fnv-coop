//! `(unplaced)/run_006ffe10` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The three words `006ffe10` copies to `+0x10` (`0119f278`, read at run time).
const DEFAULT_WORDS: u32 = 0x0119_f278;

// Translated from 006ffe10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, plain `RET`): resets the object: the pair at `+8`/`+0xC`
/// to 0, 0 ([`fn_006ffe80`]), the three words at `+0x10` from `0119f278`, the words at
/// `+0x20`, `+0x1C` and `+0x24` cleared (in that order), the byte at `+0x28` set and the one
/// at `+0x29` cleared.
pub fn fn_006ffe10(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    fn_006ffe80(e, this, 0, 0);
    for word in [0u32, 4, 8] {
        let value = e.mem.u32(DEFAULT_WORDS + word);
        e.mem.set_u32(base + 0x10 + word, value);
    }
    e.mem.set_u32(base + 0x20, 0);
    e.mem.set_u32(base + 0x1c, 0);
    e.mem.set_u32(base + 0x24, 0);
    e.mem.set_u8(base + 0x28, 1);
    e.mem.set_u8(base + 0x29, 0);
}

// Translated from 006ffe80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `RET 8`): stores `first` at `+8` and `second` at `+0xC`.
pub fn fn_006ffe80(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.mem.set_u32(this.addr() + 8, first);
    e.mem.set_u32(this.addr() + 0xc, second);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006f_fe10, fn_006ffe10(Ptr)),
        entry!(0x006f_fe80, fn_006ffe80(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reset_stores_the_defaults() {
        let mut e = Engine::new();
        e.map(0x0119_f000, 0x1000);
        e.mem.set_u32(DEFAULT_WORDS + 8, 0x77);
        let this = e.mem.alloc(0x2c);
        e.mem.write(this, &[0xcd; 0x2c]);
        e.call(0x006f_fe10, &args![this]);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x18), 0x77);
        assert_eq!(e.mem.u32(this + 0x1c), 0);
        assert_eq!(e.mem.u8(this + 0x28), 1);
        assert_eq!(e.mem.u8(this + 0x29), 0);
        assert_eq!(e.mem.u8(this + 0x2a), 0xcd);
        e.call(0x006f_fe80, &args![this, 5u32, 6u32]);
        assert_eq!(e.mem.u32(this + 0xc), 6);
    }
}
