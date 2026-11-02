//! `fallout shared/run_00541570` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00541570 (decompiled, FalloutNV.exe 1.4.0.525)
/// A byte getter (unnamed in the map; `thiscall`, no arguments): the byte at `this + 0x1C`,
/// zero-extended.
pub fn fn_00541570(e: &mut Engine, this: Ptr) -> u32 {
    u32::from(e.mem.u8(this.addr().wrapping_add(0x1c)))
}

// Translated from 00541590 (decompiled, FalloutNV.exe 1.4.0.525)
/// A byte getter (unnamed in the map; `thiscall`, no arguments): the byte at `this + 0x1F`
/// in AL (the rest of EAX is not set; not modelled).
pub fn fn_00541590(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr().wrapping_add(0x1f))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00541570, fn_00541570(Ptr) -> u32),
        entry!(0x00541590, fn_00541590(Ptr) -> u8),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_getters_read_0x1c_and_0x1f() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        e.mem.set_u8(this + 0x1c, 0x81);
        e.mem.set_u8(this + 0x1f, 0x42);
        assert_eq!(e.call(0x00541570, &args![this]).u32(), 0x81);
        assert_eq!(e.call(0x00541590, &args![this]).u8(), 0x42);
    }
}
