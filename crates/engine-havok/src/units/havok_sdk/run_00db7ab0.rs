//! `Havok SDK/run_00db7ab0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00db7ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed): constructor with three stack arguments. Sets the
/// `u16` at `+6` to 1 (the reference count of the `hkReferencedObject` layout), `+0xC` to
/// `first`, the vtable word at `+0` to 0x010d_6014, `+0x10` to `second`, `+0x14` to 1 and `+8` to
/// `third`. Returns `this`.
pub fn fn_00db7ab0(e: &mut Engine, this: Ptr, first: u32, second: u32, third: u32) -> Ptr {
    e.mem.set_u16(this.addr() + 6, 1);
    e.mem.set_u32(this.addr() + 0xc, first);
    e.mem.set_u32(this.addr(), 0x010d_6014);
    e.mem.set_u32(this.addr() + 0x10, second);
    e.mem.set_u32(this.addr() + 0x14, 1);
    e.mem.set_u32(this.addr() + 8, third);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00db7ab0, fn_00db7ab0(Ptr, u32, u32, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00db7ab0_stores_its_three_arguments() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        for offset in (0..0x20).step_by(4) {
            e.mem.set_u32(this + offset, 0xffff_ffff);
        }
        let result = e
            .call(0x00db_7ab0, &args![this, 0x11u32, 0x22u32, 0x33u32])
            .u32();
        assert_eq!(result, this);
        assert_eq!(e.mem.u32(this), 0x010d_6014);
        assert_eq!(e.mem.u16(this + 4), 0xffff);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0x33);
        assert_eq!(e.mem.u32(this + 0xc), 0x11);
        assert_eq!(e.mem.u32(this + 0x10), 0x22);
        assert_eq!(e.mem.u32(this + 0x14), 1);
        assert_eq!(e.mem.u32(this + 0x18), 0xffff_ffff);
    }
}
