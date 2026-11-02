//! `Havok SDK/run_00e39e50` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e39e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: clears the 32 bytes at `this` (`ECX`, two `MOVAPS` of zero) and
/// returns `this` (`EAX`; the decompiler shows `void`). The class is unconfirmed.
pub fn fn_00e39e50(e: &mut Engine, this: Ptr) -> Ptr {
    for word in 0..8 {
        e.mem.set_u32(this.addr() + 4 * word, 0);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e3_9e50, fn_00e39e50(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;

    /// 32 bytes are cleared, the next ones are not, and `this` comes back.
    #[test]
    fn fn_00e39e50_clears_32_bytes() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        e.mem.write(this, &[0x99; 0x40]);
        assert_eq!(e.call(0x00e3_9e50, &args![this]).u32(), this);
        assert_eq!(e.mem.bytes(this, 0x20), vec![0; 0x20]);
        assert_eq!(e.mem.bytes(this + 0x20, 0x20), vec![0x99; 0x20]);
    }
}
