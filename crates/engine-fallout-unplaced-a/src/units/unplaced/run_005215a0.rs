//! `(unplaced)/run_005215a0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005215a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x240 when `select` (a byte) is nonzero, otherwise the word at
/// +0x21C (`RET 4`). The class is not confirmed.
pub fn fn_005215a0(e: &mut Engine, this: Ptr, select: u8) -> u32 {
    if select != 0 {
        e.mem.u32(this.addr() + 0x240)
    } else {
        e.mem.u32(this.addr() + 0x21c)
    }
}

// Translated from 005215d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` at +0x21C (`RET 4`).
pub fn fn_005215d0(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x21c, value);
}

// Translated from 005215f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` at +0x220 (`RET 4`).
pub fn fn_005215f0(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x220, value);
}

// Translated from 00521610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` at +0x240 (`RET 4`).
pub fn fn_00521610(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x240, value);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0052_15a0, fn_005215a0(Ptr, u8) -> u32),
        entry!(0x0052_15d0, fn_005215d0(Ptr, u32)),
        entry!(0x0052_15f0, fn_005215f0(Ptr, u32)),
        entry!(0x0052_1610, fn_00521610(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn at_005215a0_selects_the_word() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x260);
        e.mem.set_u32(this + 0x240, 0x240);
        e.mem.set_u32(this + 0x21c, 0x21c);
        assert_eq!(e.call(0x0052_15a0, &[this, 1]).u32(), 0x240);
        assert_eq!(e.call(0x0052_15a0, &[this, 0x100]).u32(), 0x21c);
        assert_eq!(e.call(0x0052_15a0, &[this, 0]).u32(), 0x21c);
    }

    fn stores(addr: u32, offset: u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x260);
        e.call(addr, &[this, 0xdead_beef]);
        assert_eq!(e.mem.u32(this + offset), 0xdead_beef);
        assert_eq!(e.mem.u32(this + offset + 4), 0);
        assert_eq!(e.mem.u32(this + offset - 4), 0);
    }

    #[test]
    fn at_005215d0_stores_21c() {
        stores(0x0052_15d0, 0x21c);
    }

    #[test]
    fn at_005215f0_stores_220() {
        stores(0x0052_15f0, 0x220);
    }

    #[test]
    fn at_00521610_stores_240() {
        stores(0x0052_1610, 0x240);
    }
}
