//! `(unplaced)/run_0070ed80` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The global object pointer `0070ed80` tests (`011dace0`).
const OBJECT_GLOBAL: u32 = 0x011d_ace0;

// Translated from 0070ed80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): true when the object at `011dace0` exists and
/// its byte at +0x2A0 is set.
pub fn fn_0070ed80(e: &mut Engine) -> bool {
    let object = e.mem.u32(OBJECT_GLOBAL);
    object != 0 && e.mem.u8(object + 0x2a0) != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0070_ed80, fn_0070ed80() -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn needs_the_object_and_its_byte() {
        let mut e = Engine::new();
        e.mem.map(OBJECT_GLOBAL & !0xfff, 0x1000);
        e.mem.set_u32(OBJECT_GLOBAL, 0);
        assert_eq!(e.call(0x0070_ed80, &[]).u32(), 0);
        let object = e.mem.alloc(0x2b0);
        e.mem.set_u32(OBJECT_GLOBAL, object);
        assert_eq!(e.call(0x0070_ed80, &[]).u32(), 0);
        e.mem.set_u8(object + 0x2a0, 3);
        assert_eq!(e.call(0x0070_ed80, &[]).u32(), 1);
    }
}
