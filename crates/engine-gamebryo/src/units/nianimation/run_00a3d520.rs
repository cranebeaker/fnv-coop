//! `NiAnimation/run_00a3d520` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a3d520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two animation keys): 1 when the two keys' times (+0)
/// are equal (`FUCOMPP`; NaN unequal), else 0.
pub fn fn_00a3d520(e: &mut Engine, first: Ptr, second: Ptr) -> u32 {
    let a = x87_load32(e.mem.u32(first.addr()));
    let b = x87_load32(e.mem.u32(second.addr()));
    u32::from(b == a)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a3_d520, fn_00a3d520(Ptr, Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_times_equal() {
        let mut e = Engine::new();
        let (a, b) = (e.mem.alloc(4), e.mem.alloc(4));
        e.mem.set_f32(a, 0.0);
        e.mem.set_f32(b, -0.0);
        assert_eq!(e.call(0x00a3_d520, &args![a, b]).u32(), 1);
        e.mem.set_f32(b, 1.0);
        assert_eq!(e.call(0x00a3_d520, &args![a, b]).u32(), 0);
        e.mem.set_f32(a, f32::NAN);
        e.mem.set_f32(b, f32::NAN);
        assert_eq!(e.call(0x00a3_d520, &args![a, b]).u32(), 0);
    }
}
