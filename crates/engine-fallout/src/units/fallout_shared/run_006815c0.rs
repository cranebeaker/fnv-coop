//! `fallout shared/run_006815c0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006815c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns `this` itself (`mov eax, ecx`).
pub fn fn_006815c0(_e: &mut Engine, this: Ptr) -> Ptr {
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006815c0, fn_006815c0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_this() {
        let mut e = Engine::new();
        let object: Ptr = Ptr::new(e.mem.alloc(8));
        assert_eq!(e.call(0x0068_15c0, &args![object]).ptr::<()>(), object);
    }
}
