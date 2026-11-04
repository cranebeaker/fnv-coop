//! `(unplaced)/run_009d9f00` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 009d9f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. Fastcall (`ECX` = `this`): returns the address of the member at `this + 0xe0`
/// (`ADD EAX, 0xe0`), without reading memory.
pub fn fn_009d9f40(_e: &mut Engine, this: Ptr) -> Ptr {
    this.byte_add(0xe0)
}

// Translated from 009d9f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `00559450` (the word at +0) on `this + 0xd4`;
/// returns what it returns.
pub fn fn_009d9f20(e: &mut Engine, this: Ptr) -> u32 {
    e.call(0x0055_9450, &args![this.byte_add(0xd4)]).u32()
}

// Translated from 009d9f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (fastcall, `ECX` = `this`): the word at `this + 0x144`.
pub fn fn_009d9f00(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr().wrapping_add(0x144))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009d_9f00, fn_009d9f00(Ptr) -> u32),
        entry!(0x009d_9f40, fn_009d9f40(Ptr) -> Ptr),
        entry!(0x009d_9f20, fn_009d9f20(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_this_plus_0xe0() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x009d_9f40, &args![0x1000u32]).u32(), 0x10e0);
    }

    #[test]
    fn wraps_like_the_32_bit_add() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x009d_9f40, &args![0xffff_ff80u32]).u32(), 0x60);
    }

    #[test]
    fn fn_009d9f20_reads_through_the_member_at_d4() {
        let mut e = Engine::new();
        e.register(0x0055_9450, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        let this = e.mem.alloc(0xd8);
        e.mem.set_u32(this + 0xd4, 0x1234_5678);
        assert_eq!(e.call(0x009d_9f20, &args![this]).u32(), 0x1234_5678);
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    #[test]
    fn fn_009d9f00_reads_the_word_at_0x144() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x148);
        e.mem.set_u32(this + 0x144, 0xdead_beef);
        assert_eq!(e.call(0x009d_9f00, &args![this]).u32(), 0xdead_beef);
    }
}
