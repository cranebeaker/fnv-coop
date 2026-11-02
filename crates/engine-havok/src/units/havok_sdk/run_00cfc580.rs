//! `Havok SDK/run_00cfc580` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00cfc580 (decompiled, FalloutNV.exe 1.4.0.525)
/// A virtual method that does nothing (its only references are the vtable slots at `010cc8a4` and
/// `010cd560`; the class is unconfirmed): the `RET 0x14` takes five words it never reads.
pub fn fn_00cfc580(
    _e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) {
}

// Translated from 00cfc590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the vtable word `0102e388` at `this` (the base destructor body that `00cfcc30` calls).
pub fn fn_00cfc590(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0102_e388);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cf_c580, fn_00cfc580(Ptr, u32, u32, u32, u32, u32)),
        entry!(0x00cf_c590, fn_00cfc590(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The no-op changes nothing; the base destructor writes only its table word.
    #[test]
    fn the_no_op_does_nothing_and_the_destructor_writes_the_table_word() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.mem.write(this, &[0xaa; 0x10]);
        e.call(0x00cf_c580, &args![this, 1u32, 2u32, 3u32, 4u32, 5u32]);
        assert_eq!(e.mem.bytes(this, 0x10), vec![0xaa; 0x10]);
        e.call(0x00cf_c590, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0102_e388);
        assert_eq!(e.mem.u32(this + 4), 0xaaaa_aaaa);
    }
}
