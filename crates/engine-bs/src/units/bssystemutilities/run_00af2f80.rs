//! `BSSystemUtilities/run_00af2f80` (the engine map's run of unnamed functions from
//! `00af2f80`), subsystem `BSSystemUtilities`: its functions in FalloutNV.exe 1.4.0.525,
//! translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The C runtime's `strlen` (`00ec6130`) and `strcpy_s` (`00ec65a6`: destination, size,
/// source), cdecl.
const STRLEN: u32 = 0x00ec_6130;
const STRCPY_S: u32 = 0x00ec_65a6;

// Translated from 00af2f80 (disassembled, FalloutNV.exe 1.4.0.525)
/// No label (`thiscall`, one word: a destination, `RET 4`): when the string at +0xc is not
/// empty (`strlen` above 0), it is copied to the destination (`strcpy_s` with the size 0x20)
/// and 1 is answered; otherwise 0 (`AL`).
pub fn fn_00af2f80(e: &mut Engine, this: u32, destination: u32) -> u8 {
    let text = this.wrapping_add(0xc);
    if e.call(STRLEN, &args![text]).u32() > 0 {
        e.call(STRCPY_S, &args![destination, 0x20u32, text]);
        1
    } else {
        0
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00af_2f80, fn_00af2f80(u32, u32) -> u8)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copies_a_non_empty_string() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x48);
        let out = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00af_2f80, &args![this, out]).u8(), 0);
        assert_eq!(e.mem.u8(out), 0);
        e.mem.write(this + 0xc, b"save\0");
        assert_eq!(e.call(0x00af_2f80, &args![this, out]).u8(), 1);
        assert_eq!(e.mem.cstr(out), b"save".to_vec());
    }
}
