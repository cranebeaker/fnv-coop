//! `(unplaced)/run_00d87910` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d87910 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. Fastcall (`ECX` = `this`): clears the first word of the object
/// (`MOV dword ptr [ECX], 0`) and returns `this` in `EAX` (`MOV EAX, ECX` comes first). Its one
/// caller, `00d2cda0`, runs it first on its own `this` and then constructs an `hkSemaphore` (Ghidra's
/// label) in the same object.
pub fn fn_00d87910(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d8_7910, fn_00d87910(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clears_the_first_word_only_and_returns_this() {
        let mut e = Engine::new();
        let object = e.mem.alloc(16);
        for offset in [0, 4, 8, 12] {
            e.mem.set_u32(object + offset, 0xaaaa_5555);
        }
        let result = e.call(0x00d8_7910, &args![object]);
        assert_eq!(result.u32(), object);
        assert_eq!(e.mem.u32(object), 0);
        assert_eq!(e.mem.u32(object + 4), 0xaaaa_5555);
        assert_eq!(e.mem.u32(object + 12), 0xaaaa_5555);
    }
}
