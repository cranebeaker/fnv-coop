//! `fallout/interface/run_00761630` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The placement step (`006e6da0`, cdecl: the size and the address; answers where to
/// construct, or 0) and the 4-byte object's constructor (`007613f0`, `thiscall`;
/// `creditsmenu.cpp`).
const PLACE_006E6DA0: u32 = 0x006e_6da0;
const CONSTRUCT_007613F0: u32 = 0x0076_13f0;

// Translated from 00761630 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00761630 - the C++ exception-unwinding frame (FS:[0] around each construction) is not translated.
/// Unnamed (`thiscall`, `RET 8`: an array and a count; `this` is not read): constructs
/// each of the `count` 4-byte elements in place (`006e6da0(4, element)`, then
/// `007613f0` on what it answers when non-zero).
pub fn fn_00761630(e: &mut Engine, _this: Ptr, array: u32, count: u32) {
    let mut i = 0u32;
    while i < count {
        let place = e
            .call(
                PLACE_006E6DA0,
                &args![4u32, array.wrapping_add(i.wrapping_mul(4))],
            )
            .u32();
        if place != 0 {
            e.call(CONSTRUCT_007613F0, &args![place]);
        }
        i += 1;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0076_1630, fn_00761630(Ptr, u32, u32))]
}

#[cfg(test)]
mod n059_tests {
    use super::*;

    #[test]
    fn constructs_each_placed_element() {
        let mut e = Engine::new();
        // The second element gets no place.
        e.register(PLACE_006E6DA0, |_, a| {
            if a[1] == 0x5004 { 0 } else { a[1] }.into_ret()
        });
        e.register(CONSTRUCT_007613F0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0076_1630, &args![0u32, 0x5000u32, 3u32]);
        let built: Vec<_> = e
            .call_log
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == CONSTRUCT_007613F0)
            .map(|(_, w)| w[0])
            .collect();
        assert_eq!(built, vec![0x5000, 0x5008]);
    }
}
