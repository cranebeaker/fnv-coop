//! `fallout/interface/run_0073b6e0` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `009611e0` (`thiscall`, a word of the object; `playercharacter.cpp`).
const VALUE_009611E0: u32 = 0x0096_11e0;

// Translated from 0073b6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, two pointers to objects; a sort's comparison, highest first): 1, -1 or
/// 0 as `009611e0` of the first object is below, above or equal to the second's.
pub fn fn_0073b6e0(e: &mut Engine, first: u32, second: u32) -> i32 {
    let object = e.mem.u32(first);
    let a = e.call(VALUE_009611E0, &args![object]).i32();
    let object = e.mem.u32(second);
    let b = e.call(VALUE_009611E0, &args![object]).i32();
    if a < b {
        1
    } else if a > b {
        -1
    } else {
        0
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0073_b6e0, fn_0073b6e0(u32, u32) -> i32)]
}

#[cfg(test)]
mod n059_tests {
    use super::*;

    #[test]
    fn compares_highest_first() {
        let mut e = Engine::new();
        e.register(VALUE_009611E0, |_, a| a[0].into_ret());
        let holder = |e: &mut Engine, value: i32| {
            let block = e.mem.alloc(4);
            e.mem.set_i32(block, value);
            block
        };
        let (low, high) = (holder(&mut e, -3), holder(&mut e, 4));
        assert_eq!(e.call(0x0073_b6e0, &args![low, high]).i32(), 1);
        assert_eq!(e.call(0x0073_b6e0, &args![high, low]).i32(), -1);
        assert_eq!(e.call(0x0073_b6e0, &args![high, high]).i32(), 0);
    }
}
