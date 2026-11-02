//! `SpeedTree/run_00b2daf0` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `CTreeFileAccess::ParseInt` (`00b3ed80`, engine map; thiscall on the
/// file).
const PARSE_INT: u32 = 0x00b3_ed80;
/// The word the parsed integer goes to (+0x54 of `this`).
const PARSED_WORD: u32 = 0x54;

// Translated from 00b2daf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses an integer from `file` into the word at +0x54 of `this`
/// (thiscall, `RET 4`; unnamed in the engine map).
pub fn fn_00b2daf0(e: &mut Engine, this: Ptr, file: Ptr) {
    let value = e.call(PARSE_INT, &args![file]).u32();
    e.mem.set_u32(this.addr() + PARSED_WORD, value);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00b2_daf0, fn_00b2daf0(Ptr, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_the_parsed_integer_at_0x54() {
        let mut e = Engine::new();
        e.register(PARSE_INT, |_, a| (a[0] ^ 0x55).into_ret());
        let this = e.mem.alloc(0x58);
        e.call(0x00b2_daf0, &args![this, 0x0100u32]);
        assert_eq!(e.mem.u32(this + PARSED_WORD), 0x0155);
    }
}
