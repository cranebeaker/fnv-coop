//! `Havok SDK/run_00e36100` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e36100 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: `this` (`ECX`) and `other` (one stack word) hold a pointer each
/// at `+4`. When they are the same pointer, or the byte at `+0x18` of the one `other` holds is
/// not 1, the word at `other + 8` is set to 1; otherwise nothing changes. The class is
/// unconfirmed.
pub fn fn_00e36100(e: &mut Engine, this: Ptr, other: Ptr) {
    let theirs = e.mem.u32(other.addr() + 4);
    let ours = e.mem.u32(this.addr() + 4);
    if theirs == ours || e.mem.u8(theirs + 0x18) != 1 {
        e.mem.set_u32(other.addr() + 8, 1);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e3_6100, fn_00e36100(Ptr, Ptr))]
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;

    /// Same pointer: set (the byte is not looked at). Different pointers: set only when the
    /// byte at +0x18 of the other's is not 1.
    #[test]
    fn fn_00e36100_sets_the_flag_unless_the_other_object_is_marked() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        let other = e.mem.alloc(0x10);
        let ours = e.mem.alloc(0x20);
        let theirs = e.mem.alloc(0x20);
        e.mem.set_u32(this + 4, ours);
        e.mem.set_u32(other + 4, ours);
        e.mem.set_u8(ours + 0x18, 1);
        e.call(0x00e3_6100, &args![this, other]);
        assert_eq!(e.mem.u32(other + 8), 1);
        e.mem.set_u32(other + 8, 0);
        e.mem.set_u32(other + 4, theirs);
        e.mem.set_u8(theirs + 0x18, 1);
        e.call(0x00e3_6100, &args![this, other]);
        assert_eq!(e.mem.u32(other + 8), 0);
        e.mem.set_u8(theirs + 0x18, 2);
        e.call(0x00e3_6100, &args![this, other]);
        assert_eq!(e.mem.u32(other + 8), 1);
    }
}
