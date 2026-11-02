//! `Havok SDK/run_00ddb2f0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The text the function hands out (a string constant at `011b845c` in the exe image).
const TEXT: u32 = 0x011b_845c;

// Translated from 00ddb2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a virtual method, `RET 8`: stack words `out_text`, `out_length`). Stores the address of
/// the constant text at `011b845c` at `out_text` and 5 at `out_length`.
/// Virtual method: the first word is `this` (`ECX` in the exe), which the body never reads.
pub fn fn_00ddb2f0(e: &mut Engine, _this: u32, out_text: u32, out_length: u32) {
    e.mem.set_u32(out_text, TEXT);
    e.mem.set_u32(out_length, 5);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ddb2f0, fn_00ddb2f0(u32, u32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The `this` word of the virtual calls: these functions never read `ECX`.
    const VCALL_THIS: u32 = 0x7777_0000;

    #[test]
    fn the_text_address_and_length_are_stored() {
        let mut e = Engine::new();
        let out = e.mem.alloc(8);
        e.call(0x00dd_b2f0, &args![VCALL_THIS, out, out + 4]);
        assert_eq!(e.mem.u32(out), 0x011b_845c);
        assert_eq!(e.mem.u32(out + 4), 5);
    }
}
