//! `fallout/debug/sherlockinterop/bssenddebugpage.cpp` (Xbox PDB source unit), subsystem `fallout/debug`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    funcs_n086()
}

// Translated from 00c98250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSendDebugPage::GetResponceBuffer` (engine map; `thiscall`, `RET 0xc`:
/// three words it does not read): `this + 8`.
pub fn bs_send_debug_page_get_responce_buffer(
    _e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
) -> u32 {
    this.addr().wrapping_add(8)
}

/// This unit's n086 translations, by exe address.
fn funcs_n086() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00c9_8250,
        bs_send_debug_page_get_responce_buffer(Ptr, u32, u32, u32) -> u32
    )]
}

#[cfg(test)]
mod n086_tests {
    use super::*;

    #[test]
    fn responce_buffer_is_at_8() {
        let mut e = Engine::new();
        let answer = e.call(0x00c9_8250, &args![0x1000u32, 1u32, 2u32, 3u32]);
        assert_eq!(answer.u32(), 0x1008);
    }
}
